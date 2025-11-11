use std::time::SystemTime;
use postgres::{Client, NoTls, error::SqlState};
use crate::config::PostgresDBInfo;
use crate::crawled_page;

pub trait Database {
    fn get_crawled_page(self: &mut Self) -> Option<crawled_page::Crawled_page>;
    fn crawled_page_len(self: &mut Self) -> u32;
    // pub fn store_indexed_pages(self: &mut Self, pages: Vec<indexed_page::IndexedPage>) -> Result<(), Error>;
}

pub enum Error {
    SQLError(Option<SqlState>)
}

pub struct PostgresDatabase {
    client: Client
}

impl PostgresDatabase {
    pub fn new(dbinfo: &PostgresDBInfo) -> Self {
        let string: String = format!("host={} user={} password={} dbname={}", dbinfo.host, dbinfo.username, dbinfo.password, dbinfo.dbname);
        let new_client = Client::connect(&string, NoTls).unwrap();

        let db: Self = Self{
            client: new_client,
        };
        
        return db;
    }
}

impl Database for PostgresDatabase {
    fn crawled_page_len(self: &mut Self) -> u32 {
        let response = match self.client.query_one(
            "SELECT count(*) FROM crawleddata",
            &[]
        ) {
            Ok(t) => t,
            Err(_) => return 0
        };

        return response.get::<&str, i64>("count") as u32;
    }

    fn get_crawled_page(self: &mut Self) -> Option<crawled_page::Crawled_page> {
        let response = match self.client.query(
            "SELECT * FROM (SELECT * FROM crawleddata LIMIT 1) AS sq LEFT JOIN crawledwords ON sq.url=crawledwords.url",
            &[]
        ) {
            Ok(t) => t,
            Err(_) => return None
        };
        
        let mut crawled_data = crawled_page::Crawled_page {
            url: response[0].get::<&str, String>("url"),
            title: response[0].get::<&str, String>("title"),
            description: response[0].get::<&str, String>("description"),
            words: [].into()
        };

        for row in response {
            crawled_data.words.push(
                crawled_page::Word{
                    word:   row.get::<&str, String>("word"),
                    parent: row.get::<&str, String>("parent"),
                    count:  row.get::<&str, i32>("count")
                }
            );
        }

        return Some(crawled_data);
    }
}