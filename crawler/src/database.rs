use std::time::SystemTime;
use postgres::{Client, NoTls};

use crate::crawled_page;

pub struct Database {
    client: Client,
}

pub enum UsedUrlStatus {
    NewUrl,
    UrlDoesntExist,
    CannotCrawlUrl,
    CanCrawlUrl
}

impl Database {
    pub fn new() -> Self {
        let new_client = Client::connect("host=localhost user=user password=password dbname=maindb", NoTls).unwrap();

        let db: Self = Self{
            client: new_client,
        };
        
        return db;
    }

    pub fn set_schema(self: &mut Self) -> Result<String, String> {
        let result = self.client.batch_execute("
            CREATE TABLE IF NOT EXISTS CrawledData (
                url varchar(512) PRIMARY KEY,
                title varchar(128),
                description varchar(512)
            );

            CREATE TABLE IF NOT EXISTS CrawledWords (
                url varchar(512),
                word varchar(64),
                count integer,

                PRIMARY KEY (url, word)
            );

            CREATE TABLE IF NOT EXISTS URLQueue (
                url varchar(512) PRIMARY KEY,
                depth integer,
                priority integer
            );

            CREATE TABLE IF NOT EXISTS CrawledURLs (
                url varchar(512) PRIMARY KEY,
                crawl_again_at bigint
            );

            CREATE TABLE IF NOT EXISTS IndexedWords (
                url varchar(512),
                word varchar(512),
                weight integer,

                PRIMARY KEY (url, word)
            );

            CREATE TABLE IF NOT EXISTS SiteMetadata (
                url varchar(512) PRIMARY KEY,
                title varchar(512),
                description varchar(1024)
            );
        ");

        match result {
            Ok(_) => {return Ok("Done".to_string())},
            Err(t) => {return Err(format!("Error initializing db schema: {}", t))}
        };
    }

    pub fn write_crawled_page(self: &mut Self, crawledpage: &crawled_page::CrawledPage) {
        match self.client.execute(
            "INSERT INTO crawleddata VALUES ($1, $2, '');",
            &[&crawledpage.url, &crawledpage.title]
        ) {
            Ok(_) => {},
            Err(t) => panic!("Error writing to database: {}", t)
        };

        for (word, count) in crawledpage.words.iter() {
            if word.len() > 64 {
                continue;
            }

            let lowercaseword = word.to_lowercase();
            match self.client.execute(
                "INSERT INTO crawledwords VALUES ($1, $2, $3)
                ON CONFLICT (url, word) 
                DO UPDATE SET
                    count = $3",
                &[&crawledpage.url, &lowercaseword, &(*count as i32)]
            ) {
                Ok(_) => {},
                Err(t) => panic!("Error writing to database: {}", t)
            };
        }
    }

    pub fn urlqueue_get_front(self: &mut Self, pop: bool) -> Option<(String, u8)> {
        let row = match self.client.query_one(
            "SELECT * FROM urlqueue WHERE priority = 0 LIMIT 1",
            &[]
        ) {
            Ok(t) => t,
            Err(_) => match self.client.query_one(
                "SELECT * FROM urlqueue WHERE priority = 1 LIMIT 1",
                &[]
            ) {
                Ok(t) => t,
                Err(_) => return None
            }
        };

        let data: (String, u8) = (
            row.get::<&str, String>("url"), 
            row.get::<&str, i32>("depth") as u8
        );

        if pop {
            let _ = self.client.execute(
                "DELETE FROM urlqueue WHERE url = $1",
                &[&data.0]
            );
        }

        return Some(data)
    }

    pub fn urlqueue_push(self: &mut Self, url: &str, depth: u8, priority: u8) -> Result<String, String> {
        match self.client.execute(
            "INSERT INTO urlqueue VALUES ($1, $2, $3)",
            &[&url, &(depth as i32), &(priority as i32)]
        ) {
            Ok(_) => return Ok("Success".to_string()),
            Err(t) => return Err(format!("Error running urlqueue_push: {}", t))
        };
    }

    // enter a url and get the status of if its used or not
    pub fn crawledurls_status(self: &mut Self, url: &str) -> Result<UsedUrlStatus, String> {
        let used_url = match self.client.query_one(
            "SELECT * FROM crawledurls WHERE url = $1",
            &[&url]
        ) {
            Ok(t) => t,
            // bad i know
            Err(_t) => return Ok(UsedUrlStatus::UrlDoesntExist)
        };

        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("").as_secs();
        let expiry_time = used_url.get::<&str, i64>("crawl_again_at");

        if (expiry_time as u64) < now {
            return Ok(UsedUrlStatus::CanCrawlUrl);
        }
        return Ok(UsedUrlStatus::CannotCrawlUrl);
    }

    pub fn crawledurls_add(self: &mut Self, url: &str) -> Result<UsedUrlStatus, String> {
        let oneweek = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("").as_secs() + 7*86400;

        let _used_url = match self.client.execute(
            "INSERT INTO crawledurls VALUES ($1, $2)",
            &[&url, &(oneweek as i64)]
        ) {
            Ok(t) => t,
            // also bad yes i am aware
            Err(t) => {println!("crawledurls_status: {:?}", t); return Ok(UsedUrlStatus::NewUrl)}
        };

        return Ok(UsedUrlStatus::NewUrl)
    }
}