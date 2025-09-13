use std::time::{Duration, SystemTime};
use std::thread::sleep;

mod crawled_page;
mod indexed_page;
mod dictionary;
mod database;

fn main() {
    let mut db = database::Database::new();

    loop {
        let start = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("").as_millis();
        println!("Starting index...");

        match indexer_thread(&mut db) {
            Ok(_) => println!("Index successful"),
            Err(t) => println!("Index failed: {}", t),
        };

        let end = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("").as_millis();
        let dt: f64 = ((end - start) as f64) /1000.0;
        println!("Index took {} seconds", dt);
        sleep(Duration::new(20, 0));
    }
}

fn indexer_thread(db: &mut database::Database) -> Result<&'static str, &'static str>{
    for _i in 0..db.crawled_page_len() {
        let crawledpage: crawled_page::CrawledPage = match db.get_crawled_page() {
            Some(t) => t,
            None => {continue;}
        };
        
        let indexedpage = match crawledpage.index() {
            Ok(t) => t,
            Err(_) => {continue;}
        };

        db.write_indexed_metadata(&indexedpage);
    }

    return Ok("")
}


// let file = match file_result {
//     Ok(t) => t,
//     Err(_t) => {println!("Error finding file"); continue}
// };

// let file_string = match fs::read_to_string(&file.path()) {
//     Ok(t) => t,
//     Err(_t) => {println!("Error reading to string"); continue}
// };

// let mut page: crawled_page::CrawledPage = crawled_page::CrawledPage::from_string(&file_string).unwrap();

// let _ = page.filter_stop_words();

// let mut indexedpage = match page.index() {
//     Ok(t) => t,
//     Err(_t) => {println!("Error indexing page"); continue}
// };

// indexedpage.write_text(BASEPATH);

// let _ = indexedpage.write_metadata();

// let _ = fs::remove_file(&file.path());