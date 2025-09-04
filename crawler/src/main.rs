use std::time::{Duration, SystemTime};
use std::thread::sleep;
use std::collections::HashMap;
use std::collections::LinkedList;

use curl::easy::{Easy};
use tokio;

mod crawl_page;
// mod blocking;


#[tokio::main]
async fn main() {
    let mut urlqueue: LinkedList<String> = LinkedList::from([String::from("https://example.com")]);
    let mut usedurls: HashMap<String, u64> = [].into();
    
    loop {
        sleep(Duration::new(1, 0));
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
        
        let url: String = match urlqueue.pop_front() {
            Some(t) => t,
            None => {println!("Crawler ran out of urls"); break}
        };
        
        if usedurls.contains_key(&url) && *usedurls.get(&url).expect("") < now.clone().expect("").as_secs(){
            continue;
        }
        
        let crawled_urls = crawl_and_save(url.as_str());
        urlqueue.extend(crawled_urls);
        

        usedurls.insert(
            url.clone(),
            now.expect("what").as_secs() + 86400 * 7
        );

        println!("{:?}", urlqueue);
        println!("{:?}", usedurls);
    }
}

fn crawl_and_save(url: &str) -> Vec<String>{

    let mut destinations = vec![];
        
    //body
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

    let byte_array: &[u8] = out_vec.as_slice();

    let _ = crawl_page::process_out(&byte_array, url, &mut destinations);

    return destinations;
}