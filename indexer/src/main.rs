use log::{error, info, debug, LevelFilter};
use env_logger::Builder;

mod dictionary;
mod crawled_page;
mod indexed_page;
mod database;
mod config;

fn main() {
    let conf = config::Config::read_from_file("../config/config.yaml");

    Builder::new()
        // Set project's max level
        .filter(Some("indexer"), config::parse_log_level(&conf.indexer.log))
        // turn off everything else
        .filter(None, LevelFilter::Off)
        .init();

    let db: &mut dyn database::Database = &mut database::PostgresDatabase::new(&conf.database);
    let dict: &dyn dictionary::Dictionary = &dictionary::BasicDictionary::new();

    loop {
        index(db, dict);
        std::thread::sleep(std::time::Duration::from_secs(conf.indexer.time_between_indexes));
    }
}

fn index(db: &mut dyn database::Database, dict: &dyn dictionary::Dictionary) {
    info!("Index Starting");
    for _ in 0..db.crawled_page_len() {

        let crawled = db.get_crawled_page().unwrap();
        debug!("Indexing {}", crawled.url);

        let indexed: &mut dyn indexed_page::IndexedPage = &mut indexed_page::BasicIndexedPage::new();
        indexed.from_crawled_page(crawled, dict);
        match indexed.consume_into_db(db) {
            Ok(_) => {},
            Err(t) => error!("{:?}", t),
        };
    }
    info!("Index Complete");
}