use std::time::{Duration};
use std::thread::sleep;
use dotenv;

use url::Url;

mod crawled_page;
mod page_content;
mod database;


fn main() {
    let max_crawl_depth: u8 = dotenv::var("MAX_CRAWL_DEPTH").unwrap().parse().expect("Invalid crawl depth, must be an unsigned 8 bit integer");

    let dbinfo = database::DBInfo{
        host: dotenv::var("POSTGRES_DB_HOST").unwrap(),
        username: dotenv::var("POSTGRES_DB_USER").unwrap(),
        password: dotenv::var("POSTGRES_DB_PASSWORD").unwrap(),
        dbname: dotenv::var("POSTGRES_DB_DATABASE").unwrap(),
    };

    let mut db = database::Database::new(&dbinfo);
    // set db schema
    match db.set_schema() {
        Ok(_) => {}
        Err(t) => panic!("{}", t)
    };

    // ensure there is a starter url
    if db.urlqueue_count() == 0 {
        // empty, add starting url
        let _ = db.urlqueue_push("https://example.com", 0, 0);
    }

    crawler_thread(&mut db, max_crawl_depth, 1);
}

fn crawler_thread(db: &mut database::Database, max_crawl_depth: u8, crawler_id: u8) {
    loop {
        // get the url object from queue and do some preprocessing
        let raw_url_object: (String, u8) = match db.urlqueue_get_front(true) {
            Some(t) => t,
            None => {println!("Crawler ran out of urls"); break}
        };
        
        let url_object = match Url::parse(&raw_url_object.0) {
            Ok(mut t) => {filter_url(&mut t); t},
            Err(_t) => continue
        };

        let url_string: &str = url_object.as_str();
        let depth = raw_url_object.1;

        if depth > max_crawl_depth {
            continue;
        }

        drop(raw_url_object);
        
        // dont redo a url within a week
        match db.crawledurls_status(url_string).unwrap() {
            database::UsedUrlStatus::CannotCrawlUrl => {continue;}
            _ => {}
        };
        
        println!("{}: {}", depth, url_string);
        
        // fetch url as bytes
        let bytes_vec = match reqwest_url(&url_string) {
            Ok(t) => t,
            Err(t) => { println!("Failed to get {}: {}, skipping", &url_string, t); continue; }
        };
        let bytes_slice = bytes_vec.as_slice();

        // convert bytes to page content
        let pagecontent = match page_content::PageContent::from_html(&bytes_slice) {
            Ok(t) => t,
            Err(_t) => { println!("Failed to strip html from {}, skipping", &url_string); continue }
        };
        
        //append crawled urls to urlqueue, do some filtering, and increment depth
        for raw_crawled_url in &pagecontent.links {
            // Tries to parse a url. if it gets something like "/domains", it fails and then tries to join the path to the parent url,
            // so it would spit out "iana.org/domains". It double fails on fragments (good thing, they are stupid anyways). Part of me 
            // wants to make this an if statement but idiomatic code has corrupted me.
            let crawled_url = match Url::parse(raw_crawled_url) {
                Ok(mut t) => {
                    filter_url(&mut t);
                    t
                },
                Err(_t) => {
                    match url_object.join(raw_crawled_url) {
                        Ok(mut t) => {
                            filter_url(&mut t);
                            t
                        },
                        Err(_t) => continue
                    }
                }
            };
            
            match db.crawledurls_status(raw_crawled_url).unwrap() {
                database::UsedUrlStatus::CannotCrawlUrl => {continue;}
                _ => {}
            };

            if crawled_url.scheme() != "https" && crawled_url.scheme() != "http" {
                continue;
            }

            let crawled_url_host: &str = match crawled_url.domain() {
                Some(t) => t,
                None => continue
            };

            if crawled_url_host == url_object.domain().unwrap() {
                // has to be nested since we dont want depth above max being put on the queue
                if depth + 1 <= max_crawl_depth { 
                    let _ = db.urlqueue_push(crawled_url.as_str(), depth+1, 0);
                }
            } else {
                let _ = db.urlqueue_push(convert_url_to_domain(&crawled_url).as_str(), 0, 1);
            }
        }
        
        //add to usedurls
        let _ = db.crawledurls_add(url_string);
        
        //convert pagecontent to crawled url
        let crawled_page = crawled_page::CrawledPage::from_page_content(&pagecontent, url_string).unwrap();
        
        //write crawledurl to disk
        db.write_crawled_page(&crawled_page);
        
        //one page a second only on successful scrapes (good? idk)
        sleep(Duration::new(5, 0));
    }
}

fn filter_url(url: &mut url::Url) {
    url.set_fragment(None);
    url.set_query(None);
    let _ = url.set_scheme("http");
}

fn convert_url_to_domain(url: &url::Url) -> url::Url {
    let mut converted_url = url.clone();
    filter_url(&mut converted_url);
    converted_url.set_path("");
    return converted_url;
}

fn reqwest_url(url: &str) -> Result<Vec<u8>, String> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:142.0) Gecko/20100101 Firefox/142.0")
        .build()
        .unwrap();
    
    let result = match client.get(url).send() {
        Ok(t) => t,
        Err(_) => return Err("Could not get URL".to_string())
    };

    if result.status().is_redirection() {
        let redirect_to = match result.headers().get("location") {
            Some(t) => t.to_str().unwrap(),
            None => return Err("Couldnt find location header".to_string())
        };

        return reqwest_url(redirect_to);
    }

    if result.status().is_client_error() || result.status().is_server_error() {
        return Err(format!("Status Code Invalid ({})", result.status().as_str()));
    }

    let content_type = match result.headers().get("content-type") {
        Some(t) => t,
        None => return Err("No content type header".to_string())
    };

    if content_type.to_str().is_err() {
        return Err("Error verifying content type header".to_string())
    }

    if !content_type.to_str().unwrap().contains("text/html") {
        return Err("Content type is not html".to_string())
    }

    let bytes = match result.bytes() {
        Ok(t) => t,
        Err(_) => return Err("Could not get bytes".to_string())
    };

    return Ok(bytes.to_vec())
}