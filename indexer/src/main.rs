use log::{LevelFilter};
use env_logger::Builder;

mod dictionary;
mod crawled_page;
mod indexed_page;
mod database;
mod config;

fn main() {
    let conf = config::Config::read_from_file("../config.yaml");

    Builder::new()
        // Set project's max level
        .filter(Some("indexer"), config::parse_log_level(&conf.indexer.log))
        // turn off everything else
        .filter(None, LevelFilter::Off)
        .init();

    let db: &mut dyn database::Database = &mut database::PostgresDatabase::new(&conf.database);
    // let dict: 

    loop {
        index(db, &conf);
        std::thread::sleep(std::time::Duration::from_secs(20));
    }
}

fn index(db: &mut dyn database::Database, _conf: &config::Config) {
    
    for _ in 0..db.crawled_page_len() {
        let crawled = db.get_crawled_page().unwrap();
        // println!("crawled_page: {:?}\n", crawled_page.unwrap().url);

        let indexed: &mut dyn indexed_page::IndexedPage = &mut indexed_page::BasicIndexedPage::new();
        indexed.from_crawled_page(crawled);
        match indexed.consume_into_db(db) {
            Ok(_) => {},
            Err(t) => println!("{:?}", t),
        };

        return
    }
}