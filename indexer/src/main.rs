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

    index(db, &conf);
}

fn index(db: &mut dyn database::Database, conf: &config::Config) {
    
    for i in 0..db.crawled_page_len() {
        let crawled_page = db.get_crawled_page();
        println!("crawled_page: {:?}\n", crawled_page.unwrap().url);
        
    }

}