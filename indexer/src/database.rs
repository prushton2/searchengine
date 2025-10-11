use postgres::{Client, NoTls};

use crate::crawled_page;
use crate::indexed_page;

pub struct Database {
    client: Client,
}

pub struct DBInfo {
    pub host: String,
    pub username: String,
    pub password: String,
    pub dbname: String
}

impl Database {
    pub fn new(dbinfo: &DBInfo) -> Self {
        let string: String = format!("host={} user={} password={} dbname={}", dbinfo.host, dbinfo.username, dbinfo.password, dbinfo.dbname);
        let new_client = Client::connect(&string, NoTls).unwrap();

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
            "SELECT * FROM crawleddata LIMIT 1;",
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

        let delete1 = self.client.execute(
            "DELETE FROM crawleddata WHERE url = $1",
            &[&crawled_data.url]
        );

        let delete2 = self.client.execute(
            "DELETE FROM crawledwords WHERE url = $1",
            &[&crawled_data.url]
        );

        if delete1.is_err() {
            println!("Failed to delete {} from crawleddata\n   err: {:?}\n", crawled_data.url, delete1.err())
        }

        if delete2.is_err() {
            println!("Failed to delete {} from crawledwords\n   err: {:?}\n", crawled_data.url, delete2.err())
        }

        return Some(crawled_data)
    }

    pub fn write_indexed_page(self: &mut Self, indexedpage: &indexed_page::IndexedPage) -> Result<String, String> {
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
        let words: Vec<&String> = indexedpage.words.keys().collect();
        let weights: Vec<i32> = indexedpage.words.values().map(|&x| x as i32).collect();
        let urls: Vec<String> = vec![indexedpage.url.clone(); words.len()];

        self.client.execute(
            "INSERT INTO indexedwords (url, word, weight)
            SELECT * FROM UNNEST($1::text[], $2::text[], $3::int[]);",
            &[&urls, &words, &weights]
        ).unwrap();

        return Ok("".to_string())
    }
}