use std::fs;
use std::collections::HashMap;
use std::path::{PathBuf, Path};

use serde::Deserialize;
use serde_json;

#[derive(Debug, Deserialize)]
pub struct CrawledPage {
    pub version: u32,
    pub url: String,
    pub words: HashMap<String, u32>
}

const BASEPATH: &str = "../indexer_data/indexed_sites";

fn main() {
    println!("Hello, world!");
    indexer_thread();
}

fn indexer_thread() {
    let files = match fs::read_dir("../crawler_data/output") {
        Ok(t) => t,
        Err(_t) => panic!("Couldnt read directory")
    };

    println!("{:?}", files);

    for file_result in files {
        let file = match file_result {
            Ok(t) => t,
            Err(_t) => continue
        };

        let file_string = match fs::read_to_string(&file.path()) {
            Ok(t) => t,
            Err(_t) => continue
        };

        let crawled_page: CrawledPage = match serde_json::from_str(&file_string) {
            Ok(t) => t,
            Err(t) => {println!("Error deserializing json: {}", t); continue}
        };

        // todo: filter out fake words here

        index_crawled_page(&crawled_page);

        // println!("{}", file_string);
    }
}

fn index_crawled_page(crawled_page: &CrawledPage) {
    for (mut word, count) in crawled_page.words.clone().into_iter() {
        if word.len() < 2 {
            continue
        }
        word = word.to_lowercase();
        let second_byte_index = match word.char_indices().nth(2) {
            Some(t) => t.0,
            None => continue
        };

        let first_two_chars = &word[0..second_byte_index];

        let path: PathBuf = Path::new(&BASEPATH).join(first_two_chars).join(&word);
        
        println!("{:?}", path.as_os_str());

        // println!("{:?}", word);
    }
}
