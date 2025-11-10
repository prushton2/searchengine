use std::sync::{Mutex, Arc};
use std::thread;
// use log::{trace, debug, info};
use url::Url;

mod robots_txt;
mod http_request;
mod request_handler;
mod parser;
mod database;
mod config;

fn main() {
    let config = config::Config::read_from_file("../config.yaml");
    
    let httprequest: http_request::HTTPRequest = http_request::HTTPRequest::new(&config.crawler.user_agent);
    let mut database: Box<dyn database::Database + Send> = Box::new(database::PostgresDatabase::new(&config.database));
    
    let _ = database.set_schema();

    if database.urlqueue_count() == 0 {
        let _ = database.urlqueue_push(&config.crawler.seed_url, 0, 0);
    }

    let arc_mutex_db = Arc::new(Mutex::new(database));

    let mut threads = vec![];

    for i in 1..config.crawler.crawler_threads+1 {
        let arc_db = Arc::clone(&arc_mutex_db);
        let http_clone = httprequest.clone();
        threads.push(thread::spawn(move || {
            crawler_thread(arc_db, http_clone, i, config.crawler.max_crawl_depth);
        }))
    }

    for thread in threads {
        thread.join().unwrap()
    }
}

fn crawler_thread(arc_mutex_db: Arc<Mutex<Box<dyn database::Database + Send>>>, httprequest: http_request::HTTPRequest, crawler_id: i32, max_crawl_depth: i32) {
    
    let robotstxt: &mut dyn robots_txt::RobotsTXT = &mut robots_txt::RobotsTXTCrate::new(httprequest.clone());
    let requesthandler: &mut dyn request_handler::RequestHandler = &mut request_handler::SimpleRequestHandler::new(robotstxt, &httprequest);
    
    loop {

        let (url, depth) = {
            let mut database = arc_mutex_db.lock().unwrap();

            match database.urlqueue_pop_front(crawler_id) {
                Some(t) => t,
                None => { 
                    println!("{}  | No URLs", crawler_id);
                    std::thread::sleep(std::time::Duration::from_secs(5));
                    continue;
                }
            }
        };

        if depth > max_crawl_depth {
            // println!("Too Deep");
            continue;
        }

        let page_content: Vec<u8>;
        let dereferenced_url: String;
        
        match requesthandler.fetch(&url) {
            Ok(t) => {
                page_content = t.0;
                dereferenced_url = t.1;
                println!("{}  | Fetched {}", crawler_id, dereferenced_url);
            }
            Err(t) => {
                println!("{}  | Error fetching URL {}: {:?}", crawler_id, url, t);
                continue;
            },
        }

        // in normal circumstances this wouldnt run, but just incase
        // there is an edge case where a url may not lose its qstring and fragment, causing it to be re queried
        let mut database = arc_mutex_db.lock().unwrap();
        match database.crawledurls_add(&dereferenced_url) {
            database::UsedUrlStatus::NewUrl => {},
            database::UsedUrlStatus::URLExists => {
                println!("URL Already crawled: {}", dereferenced_url);
                continue;
            },
            _ => {}
        }
        
        let parsed_content: parser::ParsedData = match parser::parse_html(page_content, &dereferenced_url) {
            Ok(t) => t,
            Err(t) => { 
                println!("Bad parse: {:?}", t);
                continue
            }
        };

        let dereferenced_url_object = match Url::parse(&dereferenced_url) {
            Ok(t) => t,
            Err(_) => { 
                println!("Failed to convert dereferenced url to url object");
                continue;
            }
        };

        for raw_crawled_url in &parsed_content.urls {
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

            match database.crawledurls_status(raw_crawled_url) {
                database::UsedUrlStatus::CannotCrawlUrl => {continue;}
                _ => {}
            };

            if crawled_url.scheme() != "https" && crawled_url.scheme() != "http" {
                println!("Invalid schema on {}", raw_crawled_url);
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
                    // add the url to the queue, and set the id of the crawler responsible for it. One crawler for one domain at a time, this makes it easier to respect the crawl_delay (still need to do)
                    let _ = database.urlqueue_push(crawled_url.as_str(), depth+1, crawler_id);
                }
            } else {
                // if the domain is different, just add the domain unowned by any crawler
                let _ = database.urlqueue_push(convert_url_to_domain(&crawled_url).as_str(), 0, 0);
            }
        }

        match database.write_crawled_page(&parsed_content, &dereferenced_url) {
            Ok(_) => {},
            Err(t) => { 
                println!("Couldnt write page to db {:?}", t);
                continue; 
            }
        }

        drop(database);

        std::thread::sleep(std::time::Duration::from_secs(5));
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