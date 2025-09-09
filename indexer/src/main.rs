use std::fs;
use std::time::{Duration, SystemTime};
use std::thread::sleep;

mod crawled_page;
mod indexed_page;

const BASEPATH: &str = "../indexer_data/indexed_sites";

fn main() {
    loop {
        let start = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("").as_millis();
        println!("Starting index...");
        match indexer_thread() {
            Ok(_) => println!("Index successful"),
            Err(t) => println!("Index failed: {}", t),
        };
        let end = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("").as_millis();
        let dt: f64 = ((end - start) as f64) /1000.0;
        println!("Index took {} seconds", dt);
        sleep(Duration::new(20, 0));
    }
}

fn indexer_thread() -> Result<&'static str, &'static str>{
    let files = match fs::read_dir("../crawler_data/output") {
        Ok(t) => t,
        Err(_t) => return Err("Couldnt read directory, skipping loop")
    };

    for file_result in files {
        let file = match file_result {
            Ok(t) => t,
            Err(_t) => continue
        };

        let file_string = match fs::read_to_string(&file.path()) {
            Ok(t) => t,
            Err(_t) => continue
        };

        let page: crawled_page::V1 = crawled_page::V1::from_string(&file_string).unwrap();

        // todo: filter out fake words here

        let indexed_page = match page.index() {
            Ok(t) => t,
            Err(_t) => continue
        };

        indexed_page.write_text(BASEPATH);

        let result = indexed_page.write_metadata();
        println!("index write: {:?}", result)
    }

    return Ok("")
}


