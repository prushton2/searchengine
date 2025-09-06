use std::time::{Duration, SystemTime};
use std::thread::sleep;
use std::collections::HashMap;
use std::collections::LinkedList;
use std::fs;
use std::io::Write;

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
        
        let url: (String, u8) = match urlqueue.pop_front() {
            Some(t) => t,
            None => {println!("Crawler ran out of urls"); break}
        };
        
        // dont redo a url within a week
        if usedurls.contains_key(&url.0) && *usedurls.get(&url.0).expect("") > now.clone().expect("").as_secs(){
            println!("Used {}", url.0);
            continue;
        }
        
        //crawl it and store the crawled urls
        let crawled_urls = crawl_and_save(url.0.as_str());
        
        for crawled_url in crawled_urls {
            if !usedurls.contains_key(&crawled_url.0) {
                urlqueue.push_back(crawled_url);
            }
        }

        usedurls.insert(
            url.0.clone(),
            now.expect("what").as_secs() + 86400 * 7
        );
        
        //save progress
        write_mem_to_file(&urlqueue, &usedurls);

        //one page a second only on successful scrapes (good? idk)
		sleep(Duration::new(1, 0));
    }
}

fn crawl_and_save(url: &str) -> Vec<(String, u8)>{

    let mut destinations: Vec<(String, u8)> = vec![];
    let mut out_vec = Vec::new();

    // curl is scoped to ensure the borrow of out_vec is released before other use
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

    let byte_array: &[u8] = out_vec.as_slice();

    // create the crawled page struct
    let crawled_page = crawl_page::crawl_page(&byte_array, url).unwrap();

    let _ = write_crawled_page_to_file(&crawled_page);

    return destinations;
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