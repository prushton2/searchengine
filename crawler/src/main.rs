use std::time::{Duration, SystemTime};
use std::thread::sleep;
use std::collections::HashMap;
use std::collections::LinkedList;
use std::fs;
use std::io::Write;

use serde_json;
use curl::easy::{Easy};

mod crawl_page;

fn main() {
    let mut urlqueue: LinkedList<String> = LinkedList::from([String::from("https://example.com")]);
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
            Err(_t) => LinkedList::from([String::from("https://example.com")])
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

    crawl_thread(&mut urlqueue, &mut usedurls);
}

fn crawl_thread(urlqueue: &mut LinkedList<String>, usedurls: &mut HashMap<String, u64>) {
    loop {
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
        
        let url: String = match urlqueue.pop_front() {
            Some(t) => t,
            None => {println!("Crawler ran out of urls"); break}
        };
        
        // dont redo a url within a week
        if usedurls.contains_key(&url) && *usedurls.get(&url).expect("") > now.clone().expect("").as_secs(){
            println!("Used {}", url);
            continue;
        }
        
        //crawl it and store the crawled urls
        let crawled_urls = crawl_and_save(url.as_str());
        urlqueue.extend(crawled_urls);
        
        usedurls.insert(
            url.clone(),
            now.expect("what").as_secs() + 86400 * 7
        );
        
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
        
        //one page a second only on successful scrapes (good? idk)
		sleep(Duration::new(1, 0));
    }
}

fn crawl_and_save(url: &str) -> Vec<String>{

    let mut destinations = vec![];
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

    let _ = crawl_page::process_out(&byte_array, url, &mut destinations);

    return destinations;
}