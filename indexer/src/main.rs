use std::fs;
// use std::collections::HashMap;
// use std::path::{PathBuf, Path};


mod crawled_page;

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

        let page: crawled_page::CrawledPage = crawled_page::CrawledPage::from_string(&file_string).unwrap();

        // todo: filter out fake words here

        page.save(BASEPATH);

        // println!("{}", file_string);
    }
}


