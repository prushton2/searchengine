use std::thread;
use url::Url;
use log::{warn, trace, debug, info, error, LevelFilter};
use env_logger::Builder;

mod robots_txt;
mod http_request;
mod request_handler;
mod parser;
mod database;
mod config;

fn main() {
    let conf = config::Config::read_from_file("../config/config.yaml");

    Builder::new()
        // Set project's max level
        .filter(Some("crawler"), config::parse_log_level(&conf.crawler.log))
        // turn off everything else
        .filter(None, LevelFilter::Off)
        .init();

    info!("Initializing {} crawler threads with a max depth of {}, and a seed url of {}", conf.crawler.crawler_threads, conf.crawler.max_crawl_depth, conf.crawler.seed_url);
    
    let httprequest: http_request::HTTPRequest = http_request::HTTPRequest::new(&conf.crawler.user_agent);
    let database: &mut dyn database::Database = &mut database::PostgresDatabase::new(&conf.database);
    
    match database.set_schema() {
        Ok(()) => {info!("Initialized DB Schema");}
        Err(_) => {}
    };

    if database.urlqueue_count() == 0 {
        let _ = database.urlqueue_push(&conf.crawler.seed_url, 0, 0);
        info!("pushed seed url to queue");
    }

    // drop(database);
    
    let mut threads = vec![];

    // I start at 1 because a url with a crawler id 0 in the database means it unassigned to a crawler
    for i in 1..conf.crawler.crawler_threads+1 {
        let http_clone = httprequest.clone();
        let db_conf = conf.database.clone();
        threads.push(thread::spawn(move || {
            crawler_thread(db_conf, http_clone, i, conf.crawler.max_crawl_depth);
        }))
    }

    for thread in threads {
        thread.join().unwrap()
    }
}

// a crawler thread handles one domain at a time. once done, it grabs a new domain unassigned to a crawler from the queue
fn crawler_thread(db_conf: config::PostgresDBInfo, httprequest: http_request::HTTPRequest, crawler_id: i32, max_crawl_depth: i32) {
    
    let robotstxt: &mut dyn robots_txt::RobotsTXT = &mut robots_txt::RobotsTXTCrate::new(httprequest.clone());
    let requesthandler: &mut dyn request_handler::RequestHandler = &mut request_handler::SimpleRequestHandler::new(robotstxt, &httprequest);
    let database: &mut dyn database::Database = &mut database::PostgresDatabase::new(&db_conf);

    // if we get 5 loops with no urls, exit
    let mut no_urls_count = 0;
    
    while no_urls_count < 5 {

        let (url, depth) = {

            match database.urlqueue_pop_front(crawler_id) {
                Some(t) => {
                    no_urls_count = 0;
                    t
                },
                None => {
                    if database.urlqueue_count() == 0 {
                        no_urls_count += 1;
                        warn!("{}  | No URLs in queue ({}/5)", crawler_id, no_urls_count);
                    }
                    std::thread::sleep(std::time::Duration::from_secs(5));
                    continue;
                }
            }
        };

        if depth > max_crawl_depth {
            trace!("Too Deep");
            continue;
        }

        let page_content: Vec<u8>;
        let dereferenced_url: String;
        
        match requesthandler.fetch(&url) {
            Ok(t) => {
                page_content = t.0;
                dereferenced_url = t.1;
                debug!("{}  | Fetched {}", crawler_id, dereferenced_url);
            }
            Err(t) => {
                debug!("{}  | Error fetching URL {}: {:?}", crawler_id, url, t);
                continue;
            },
        }

        // in normal circumstances this wouldnt run, but just incase
        // there is an edge case where a url may not lose its qstring and fragment, causing it to be re queried
        match database.crawledurls_add(&dereferenced_url) {
            database::UsedUrlStatus::NewUrl => {},
            database::UsedUrlStatus::URLExists => {
                debug!("URL Already crawled: {}", dereferenced_url);
                continue;
            },
            _ => {}
        }
        
        let parsed_content: parser::ParsedData = match parser::parse_html(page_content, &dereferenced_url) {
            Ok(t) => t,
            Err(t) => { 
                trace!("Bad parse: {:?}", t);
                continue
            }
        };

        let dereferenced_url_object = match Url::parse(&dereferenced_url) {
            Ok(t) => t,
            Err(_) => { 
                trace!("Failed to convert dereferenced url to url object");
                continue;
            }
        };

        trace!("{}  | Finished post fetch", crawler_id);

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

            match database.crawledurls_status(crawled_url.as_str()) {
                database::UsedUrlStatus::CannotCrawlUrl => {continue;}
                _ => {}
            };

            if crawled_url.scheme() != "https" && crawled_url.scheme() != "http" {
                debug!("Invalid schema on {}", crawled_url.as_str());
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

        trace!("{}  | Finished URL parsing", crawler_id);

        match database.write_crawled_page(&parsed_content, &dereferenced_url) {
            Ok(_) => {},
            Err(database::Error::SQLError(Some(t))) => {
                warn!("{}  | Couldnt write {} to db {:?}", crawler_id, dereferenced_url, t);
                continue; 
            },
            Err(t) => {
                warn!("{}  | Couldnt write {} to db {:?}", crawler_id, dereferenced_url, t);
                continue;
            }
        }

        trace!("{}  | Finished crawling page", crawler_id);

        std::thread::sleep(std::time::Duration::from_secs(5));
    }

    error!("Crawler {} had no urls for 5 loops, exiting...", crawler_id);
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