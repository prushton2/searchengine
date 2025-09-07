use std::time::{Duration, SystemTime};
use std::thread::sleep;
use std::collections::HashMap;
use std::collections::LinkedList;
use std::fs;
use std::io::Write;

use url::Url;
use serde_json;
use curl::easy::{Easy};

mod crawl_page;

const MAX_CRAWL_DEPTH: u8 = 5;

fn main() {
    let mut urlqueue: LinkedList<(String, u8)> = LinkedList::from([(String::from("https://example.com"), 0)]);
    let mut usedurls: HashMap<String, u64> = [].into();
    if !fs::exists("../crawler_data").unwrap() {
        let _ = fs::create_dir("../crawler_data");
        let _ = fs::create_dir("../crawler_data/output");
        let _ = fs::File::create("../crawler_data/urlqueue.json");
        let _ = fs::File::create("../crawler_data/usedurls.json");
    } else {
        let urlqueue_file = match fs::read_to_string("../crawler_data/urlqueue.json") {
            Ok(t) => t,
            Err(t) => panic!("{}", t)
        };

        urlqueue = match serde_json::from_str(&urlqueue_file) {
            Ok(t) => t,
            Err(_t) => LinkedList::from([(String::from("https://example.com"), 0)])
        };

        let usedurls_file = match fs::read_to_string("../crawler_data/usedurls.json") {
            Ok(t) => t,
            Err(t) => panic!("{}", t)
        };
        
        usedurls = match serde_json::from_str(&usedurls_file) {
            Ok(t) => t,
            Err(_t) => [].into()
        };
    }

    crawler_thread(&mut urlqueue, &mut usedurls);
}

fn crawler_thread(urlqueue: &mut LinkedList<(String, u8)>, usedurls: &mut HashMap<String, u64>) {
    loop {
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
        
        // get the url object from queue and do some preprocessing
        let raw_url_object: (String, u8) = match urlqueue.pop_front() {
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
        
        // dont redo a url within a week
        if usedurls.contains_key(url_string) && *usedurls.get(url_string).expect("") > now.clone().expect("").as_secs(){
            println!("Used {}", url_string);
            continue;
        }

        // fetch url as bytes
        let bytes_vec = match fetch_url(&url_string) {
            Ok(t) => t,
            Err(_t) => { println!("Failed to get {}, skipping", &url_string); continue; }
        };
        let bytes_slice = bytes_vec.as_slice();

        // convert bytes to page content
        let page_content = match crawl_page::strip_html(&bytes_slice) {
            Ok(t) => t,
            Err(_t) => { println!("Failed to strip html from {}, skipping", &url_string); continue }
        };
        
        //append crawled urls to urlqueue, do some filtering, and increment depth
        for raw_crawled_url in &page_content.links {


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
            
            if usedurls.contains_key(raw_crawled_url) {
                continue;
            }

            if url_object.scheme() != "https" && url_object.scheme() != "http" {
                continue;
            }

            let crawled_url_host: &str = match crawled_url.domain() {
                Some(t) => t,
                None => panic!("No host in {}", crawled_url.as_str())
            };

            let new_depth;
            
            if crawled_url_host == url_object.domain().unwrap() {
                new_depth = depth + 1;
            } else {
                new_depth = 0;
            }
    
            // only append if depth isnt too deep
            if depth <= MAX_CRAWL_DEPTH {
                urlqueue.push_back((crawled_url.as_str().to_string(), new_depth));
            }

        }
        
        //add to usedurls
        usedurls.insert(url_string.to_string(), now.expect("").as_secs() + 7 * 86400);
        
        //convert pagecontent to crawled url
        let crawled_page = crawl_page::create_crawled_page_object(&page_content, url_string).unwrap();
        
        //write crawledurl to disk
        let _ = write_crawled_page_to_file(&crawled_page);
        
        //save progress
        write_mem_to_file(&urlqueue, &usedurls);
        
        //one page a second only on successful scrapes (good? idk)
        sleep(Duration::new(5, 0));
    }
}

fn filter_url(url: &mut url::Url) {
    url.set_fragment(None);
}

fn fetch_url(url: &str) -> Result<Vec<u8>, &'static str> {
    let mut out_vec = Vec::new();
    {
        let mut curl = Easy::new();
        curl.url(url).unwrap();
        curl.useragent("Mozilla/5.0 (X11; Linux x86_64; rv:142.0) Gecko/20100101 Firefox/142.0").unwrap();

        let mut transfer = curl.transfer();
        
        let _ = transfer.write_function(|bytes| {
            out_vec.extend_from_slice(bytes);
            return Ok(bytes.len())
        }).unwrap();
        
        let result = transfer.perform();
        if result.is_err() {
            return Err("Transfer failed");
        }
    }
    
    return Ok(out_vec);
}

fn write_mem_to_file(urlqueue: &LinkedList<(String, u8)>, usedurls: &HashMap<String, u64>) {
    let urlqueue_serialized = match serde_json::to_string(&urlqueue) {
        Ok(t) => t,
        Err(t) => panic!("{}", t)
    };
    
    let usedurls_serialized = match serde_json::to_string(&usedurls) {
        Ok(t) => t,
        Err(t) => panic!("{}", t)
    };
    
    let mut urlqueue_file = match fs::File::create("../crawler_data/urlqueue.json") {
        Ok(t) => t,
        Err(t) => panic!("{}", t)
    };
    let mut usedurls_file = match fs::File::create("../crawler_data/usedurls.json") {
        Ok(t) => t,
        Err(t) => panic!("{}", t)
    };
    
    let _ = urlqueue_file.write_all(urlqueue_serialized.as_bytes());
    let _ = usedurls_file.write_all(usedurls_serialized.as_bytes());
}

fn write_crawled_page_to_file(crawled_page: &crawl_page::CrawledPage) -> Result<&'static str, &'static str> {
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("").as_secs();
    let filename = ["../crawler_data/output/", now.to_string().as_str(), ".json"].concat();

    let file_result = fs::File::create(filename);
    let serialized = serde_json::to_string(&crawled_page);

    if serialized.is_err() {
        return Err("Serialization Failed");
    }

    if file_result.is_err() {
        return Err("Error opening file");
    }

    let mut file = file_result.unwrap();

    let _ = file.write_all(serialized.unwrap().as_bytes());
    return Ok("File Written")
}