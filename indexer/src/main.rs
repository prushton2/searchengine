use std::time::{Duration, SystemTime};
use std::thread::sleep;

mod crawled_page;
mod indexed_page;
mod dictionary;
mod database;

fn main() {
    let dbinfo = database::DBInfo{
        host: dotenv::var("POSTGRES_DB_HOST").unwrap(),
        username: dotenv::var("POSTGRES_DB_USER").unwrap(),
        password: dotenv::var("POSTGRES_DB_PASSWORD").unwrap(),
        dbname: dotenv::var("POSTGRES_DB_DATABASE").unwrap(),
    };

    let mut db = database::Database::new(&dbinfo);

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
        let mut crawledpage: crawled_page::CrawledPage = match db.get_crawled_page() {
            Some(t) => t,
            None => {continue;}
        };

        let _ = crawledpage.filter_stop_words();
        
        let indexedpage = match crawledpage.index() {
            Ok(t) => t,
            Err(_) => {continue;}
        };

        let _ = db.write_indexed_metadata(&indexedpage);
    }

    return Ok("")
}