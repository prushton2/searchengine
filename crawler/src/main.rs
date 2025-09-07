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

    println!("{:?}", urlqueue);

    crawler_thread(&mut urlqueue, &mut usedurls);
}

fn crawler_thread(urlqueue: &mut LinkedList<(String, u8)>, usedurls: &mut HashMap<String, u64>) {
    loop {
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
        
        let url: (String, u8) = match urlqueue.pop_front() {
            Some(t) => t,
            None => {println!("Crawler ran out of urls"); break}
        };

        println!("Crawling url {}", &url.0);
        
        // dont redo a url within a week
        if usedurls.contains_key(&url.0) && *usedurls.get(&url.0).expect("") > now.clone().expect("").as_secs(){
            println!("Used {}", url.0);
            continue;
        }

        //fetch url
        let bytes_vec = fetch_url(&url.0).unwrap();
        let bytes_slice = bytes_vec.as_slice();

        //convert bytes to page content
        let page_content = match crawl_page::strip_html(&bytes_slice) {
            Ok(t) => t,
            Err(_t) => { println!("Failed to strip html from {}, skipping", &url.0); continue }
        };

        //append crawled urls to urlqueue and increment depth
        let host: String = get_host_from_url(&url.0).unwrap();

        for crawled_url in &page_content.links {
            let mut depth = url.1;

            if usedurls.contains_key(crawled_url) {
                continue;
            }

            // resolve things like "/domains" to "iana.org/domains"
            let formatted_url; 
            if crawled_url.chars().nth(0) == Some('/') {
                formatted_url = ["http://", &host, &crawled_url].concat();
            } 
            // ignore heading links, these dont tell us anything we dont already know
            else if crawled_url.chars().nth(0) == Some('#') { 
                continue;
            } 
            else {
                formatted_url = crawled_url.to_string();
            }

            // increment depth
            let formatted_url_host: String = get_host_from_url(&formatted_url).unwrap();
            if formatted_url_host == host {
                depth += 1;
            } else {
                depth = 0;
            }

            // only append if depth isnt too deep
            if depth <= MAX_CRAWL_DEPTH {
                urlqueue.push_back((formatted_url.clone(), depth));
            }
        }

        //add to usedurls
        usedurls.insert(url.0.clone(), now.expect("").as_secs() + 7 * 86400);

        //convert pagecontent to crawled url
        let crawled_page = crawl_page::create_crawled_page_object(&page_content, &url.0).unwrap();

        //write crawledurl to disk
        let _ = write_crawled_page_to_file(&crawled_page);

        //save progress
        write_mem_to_file(&urlqueue, &usedurls);

        //one page a second only on successful scrapes (good? idk)
		sleep(Duration::new(5, 0));
    }
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
        
        transfer.perform().unwrap();
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

fn get_host_from_url(url: &str) -> Result<String, &'static str> {
    let parsed = match Url::parse(url) {
        Ok(t) => t,
        Err(_t) => return Err("Error parsing url string. Is it valid?")
    };

    return match parsed.host_str() {
        Some(t) => Ok(t.to_string()),
        None => Err("No hostname in url")
    };
}