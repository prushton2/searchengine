use std::time::{Duration};
use std::thread::sleep;
use std::str;
use dotenv;

use url::Url;
use robotstxt::DefaultMatcher;

mod crawled_page;
mod page_content;
mod database;

static USER_AGENT: &str = "search.prushton.com/1.0 (https://github.com/prushton2/searchengine)";

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
        let _ = db.urlqueue_push("https://en.wikipedia.org/wiki/Banana_republic", 0, 0);
    }
    
    println!("Crawler running");

    crawler_thread(&mut db, max_crawl_depth, 1);
}

fn crawler_thread(db: &mut database::Database, max_crawl_depth: u8, crawler_id: i32) {
    let mut previous_domain: String = String::from("");
    let mut robotstxt: String = String::from("");
    let environment = dotenv::var("ENVIRONMENT").unwrap();
    
    loop {
        let mut matcher = DefaultMatcher::default();

        // get the url object from queue and do some preprocessing
        let raw_url_object: (String, u8) = match db.urlqueue_pop_front(crawler_id) {
            Some(t) => t,
            None => {println!("Crawler ran out of urls"); break}
        };
        
        let url_object = match Url::parse(&raw_url_object.0) {
            Ok(mut t) => {filter_url(&mut t); t},
            Err(_t) => continue
        };

        let url_string: &str = url_object.as_str();
        let depth = raw_url_object.1;
        drop(raw_url_object);
        
        if depth > max_crawl_depth {
            continue;
        }

        // if the url changed, we need to refetch robots.txt
        if url_object.domain() != Some(previous_domain.as_str()) {
            
            let mut robots_path = url_object.clone();
            robots_path.set_path("/robots.txt");
            robots_path.set_query(None);
            robots_path.set_fragment(None);
            
            let robots_bytes: Vec<u8> = match reqwest_url(robots_path.as_str()) {
                Ok(t) => t.0,
                Err(_) => "user-agent: *\ndisallow:".as_bytes().to_owned()
            };
            
            robotstxt = match str::from_utf8(&robots_bytes) {
                Ok(t) => t.to_string(),
                Err(_) => "user-agent: *\ndisallow:".into(),
            };

            if environment == "dev" {
                println!("New robots.txt file fetched from {} for crawler id {}", robots_path.domain().expect("Bad url_object host"), crawler_id);
            }
            previous_domain = robots_path.domain().expect("Bad url_object host").to_string();
        }

        // check if we are allowed to crawl here
        if !matcher.one_agent_allowed_by_robots(&robotstxt, USER_AGENT, &url_string) {
            continue
        }

        // dont redo a url within the defined timeframe
        match db.crawledurls_status(url_string).unwrap() {
            database::UsedUrlStatus::CannotCrawlUrl => {continue;}
            _ => {}
        };
        
        if environment == "dev" {
            println!("{}: {}", depth, url_string);
        }
        
        // fetch url as bytes
        let response: (Vec<u8>, String) = match reqwest_url(&url_string) {
            Ok(t) => t,
            Err(t) => { println!("Failed to get {}: {}, skipping", &url_string, t); continue; }
        };
        let bytes_slice = response.0.as_slice();

        let dereferenced_url_object = match Url::parse(&response.1) {
            Ok(t) => t,
            Err(_) => { println!("Somehow the redirect url {} was valid enough to fetch, but isnt valid enough for the Url crate", response.1); continue;}
        };

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
                    match dereferenced_url_object.join(raw_crawled_url) {
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

            // no host, no index
            let crawled_url_host: &str = match crawled_url.domain() {
                Some(t) => t,
                None => continue
            };

            if crawled_url_host == dereferenced_url_object.domain().unwrap() {
                // has to be nested since we dont want depth above max being put on the queue
                if depth + 1 <= max_crawl_depth {
                    // add the url to the queue, and set the id of the crawler responsible for it. One crawler crawl one webpage, this makes it easier to respect the crawl_delay
                    let _ = db.urlqueue_push(crawled_url.as_str(), depth+1, crawler_id);
                }
            } else {
                // if the domain is different, just add the domain unowned by any crawler
                let _ = db.urlqueue_push(convert_url_to_domain(&crawled_url).as_str(), 0, 0);
            }
        }
        
        // add the url to the list of crawled urls
        let _ = db.crawledurls_add(dereferenced_url_object.as_str());
        
        // convert pagecontent to crawled url
        let crawledpage = crawled_page::CrawledPage::from_page_content(&pagecontent, url_string).unwrap();
        
        // write crawledurl to disk
        db.write_crawled_page(&crawledpage);
        
        // one page every 5 seconds only on successful scrapes (good? idk?)
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

fn reqwest_url(url: &str) -> Result<(Vec<u8>, String), String> {
    let client = reqwest::blocking::Client::builder()
        .user_agent(USER_AGENT)
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

    // returning the url lets us know what the actual url is when dereferencing 3XX Urls
    return Ok((bytes.to_vec(), url.to_owned()))
}