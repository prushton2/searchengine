use postgres::{Client, NoTls};

use crate::crawled_page;
use crate::indexed_page;

pub struct Database {
    client: Client,
}

impl Database {
    pub fn new() -> Self {
        let new_client = Client::connect("host=localhost user=user password=password dbname=maindb", NoTls).unwrap();

        let db: Self = Self{
            client: new_client,
        };
        
        return db;
    }

    pub fn crawled_page_len(self: &mut Self) -> u32 {
        let response = match self.client.query_one(
            "SELECT count(*) FROM crawleddata",
            &[]
        ) {
            Ok(t) => t,
            Err(_) => return 0
        };

        return response.get::<&str, i64>("count") as u32;
    }

    pub fn get_crawled_page(self: &mut Self) -> Option<crawled_page::CrawledPage> {
        let crawled_data_response = match self.client.query_one(
            "SELECT * FROM crawleddata LIMIT 1",
            &[]
        ) {
            Ok(t) => t,
            Err(_) => return None
        };

        let mut crawled_data: crawled_page::CrawledPage = crawled_page::CrawledPage{
            url: crawled_data_response.get::<&str, String>("url"),
            title: crawled_data_response.get::<&str, String>("title"),
            description: crawled_data_response.get::<&str, String>("description"),
            words: [].into()
        };

        let words = match self.client.query(
            "SELECT * FROM crawledwords WHERE url = $1",
            &[&crawled_data.url]
        ) {
            Ok(t) => t,
            Err(_) => {vec![]}
        };

        for row in words {
            crawled_data.words.insert(
                row.get::<&str, String>("word"),
                row.get::<&str, i32>("count") as u64
            );
        }

        let _ = self.client.execute(
            "DELETE FROM crawleddata WHERE url = $1",
            &[&crawled_data.url]
        );

        let _ = self.client.execute(
            "DELETE FROM crawledwords WHERE url = $1",
            &[&crawled_data.url]
        );

        return Some(crawled_data)
    }

    pub fn write_indexed_metadata(self: &mut Self, indexedpage: &indexed_page::IndexedPage) -> Result<String, String> {
        // write metadata
        match self.client.execute(
            "INSERT INTO sitemetadata VALUES ($1, $2, $3)
            ON CONFLICT (url)
            DO UPDATE SET
                title = $2,
                description = $3",
            &[&indexedpage.url, &indexedpage.title, &indexedpage.description]
        ) {
            Ok(_) => {},
            Err(t) => return Err(format!("{:?}", t))
        };

        // remove existing indexed data about site
        match self.client.execute(
            "DELETE FROM indexedwords WHERE url = $1",
            &[&indexedpage.url]
        ) {
            Ok(_) => {},
            Err(t) => {panic!("{:?}", t);},
        };

        // add words
        for (word, weight) in indexedpage.words.iter() {
            self.client.execute(
                "INSERT INTO indexedwords VALUES ($1, $2, $3)",
                &[&indexedpage.url, &word, &(*weight as i32)]
            ).unwrap();
        }

        return Ok("".to_string())
    }
}