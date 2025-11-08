use std::time::SystemTime;
use postgres::{Client, NoTls, error::SqlState};
use crate::parser;

pub trait Database {
    fn set_schema(self: &mut Self) -> Result<(), Error>;
    fn write_crawled_page(self: &mut Self, page: &parser::ParsedData, url: &String) -> Result<(), Error>;
    fn urlqueue_count(self: &mut Self) -> i64;
    fn urlqueue_pop_front(self: &mut Self, crawler_id: i32) -> Option<(String, i32)>;
    fn urlqueue_push(self: &mut Self, url: &str, depth: i32, crawler_id: i32) -> Result<String, String>;
    fn crawledurls_status(self: &mut Self, url: &str) -> UsedUrlStatus;
    fn crawledurls_add(self: &mut Self, url: &str) -> UsedUrlStatus;
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
    SQLError(Option<SqlState>)
}

pub enum UsedUrlStatus {
    NewUrl,
    UrlDoesntExist,
    URLExists,
    CannotCrawlUrl,
    CanCrawlUrl
}

pub struct PostgresDatabase {
    client: Client
}

pub struct DBInfo {
    pub host: String,
    pub username: String,
    pub password: String,
    pub dbname: String
}


impl PostgresDatabase {
    pub fn new(dbinfo: &DBInfo) -> Self {
        let string: String = format!("host={} user={} password={} dbname={}", dbinfo.host, dbinfo.username, dbinfo.password, dbinfo.dbname);
        let new_client = Client::connect(&string, NoTls).unwrap();

        let db: Self = Self{
            client: new_client,
        };
        
        return db;
    }
}

impl Database for PostgresDatabase {
    fn set_schema(self: &mut Self) -> Result<(), Error> {
        let result = self.client.batch_execute("
            CREATE TABLE IF NOT EXISTS CrawledData (
                url varchar(512) PRIMARY KEY,
                title varchar(512),
                description varchar(1024)
            );

            CREATE TABLE IF NOT EXISTS CrawledWords (
                url varchar(512),
                parent varchar(512),
                word varchar(64),
                count integer,

                PRIMARY KEY (url, word, parent)
            );

            CREATE TABLE IF NOT EXISTS URLQueue (
                url varchar(512) PRIMARY KEY,
                depth integer,
                crawler_id integer
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
            Ok(_) => {return Ok(())},
            Err(t) => {return Err(Error::SQLError(t.code().cloned()))}
        };
    }


    fn write_crawled_page(self: &mut Self, page: &parser::ParsedData, url: &String) -> Result<(), Error> {
        let mut urls: Vec<String> = vec![];
        let mut words: Vec<String> = vec![];
        let mut parents: Vec<String> = vec![];
        let mut counts: Vec<i32> = vec![];

        for word in page.words.iter() {
            if word.word.len() > 64 {
                continue;
            }

            urls.push(url.clone());
            words.push(word.word.clone());
            parents.push(word.parent.clone());
            counts.push(word.count as i32);
            
        }
        
        match self.client.execute(
            "INSERT INTO crawledwords (url, parent, word, count)
            SELECT * FROM UNNEST($1::text[], $2::text[], $3::text[], $4::int[])",
            &[&urls, &parents, &words, &counts]
        ) {
            Ok(_) => {},
            Err(t) => return Err(Error::SQLError(t.code().cloned()))
        };
        /*  
            This is at the end because i cant be bothered to lock the db
            tldr: An issue appears whe the crawleddata gets written -> indexer pops new crawleddata entry ->
            indexer only gets some words, the rest arent written yet -> 
            orphaned words build up over time

            I solved this by ensuring the words are written before giving them a parent,
            not ideal but it works
        */

        match self.client.execute(
            "INSERT INTO crawleddata VALUES ($1, $2, $3);",
            &[&url, &page.title, &page.description]
        ) {
            Ok(_) => {},
            Err(t) => return Err(Error::SQLError(t.code().cloned()))
        };
        return Ok(())
    }

    fn urlqueue_count(self: &mut Self) -> i64 {
        let row = match self.client.query_one(
            "SELECT COUNT(*) FROM urlqueue",
            &[]
        ) {
            Ok(t) => t,
            Err(_) => return 0
        };
        return row.get::<&str, i64>("count")
    }

    fn urlqueue_pop_front(self: &mut Self, crawler_id: i32) -> Option<(String, i32)> {
        // get a url owned by the callee
        let row = match self.client.query_one(
            "SELECT * FROM urlqueue WHERE crawler_id=$1 LIMIT 1",
            &[&crawler_id]
        ) {
            Ok(t) => t,
            // if fails, get an unowned url
            Err(_) => match self.client.query_one(
                "SELECT * FROM urlqueue WHERE crawler_id=0 LIMIT 1",
                &[]
            ) {
                Ok(t) => t,
                Err(_) => return None
            }
        };

        let data: (String, i32) = (
            row.get::<&str, String>("url"), 
            row.get::<&str, i32>("depth")
        );

        let _ = self.client.execute(
            "DELETE FROM urlqueue WHERE url = $1",
            &[&data.0]
        );

        return Some(data)
    }

    fn urlqueue_push(self: &mut Self, url: &str, depth: i32, crawler_id: i32) -> Result<String, String> {
        match self.client.execute(
            "INSERT INTO urlqueue VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
            &[&url, &depth, &crawler_id]
        ) {
            Ok(_) => return Ok("Success".to_string()),
            Err(t) => return Err(format!("Error running urlqueue_push: {}", t))
        };
    }

    fn crawledurls_status(self: &mut Self, url: &str) -> UsedUrlStatus {
        let used_url = match self.client.query_one(
            "SELECT * FROM crawledurls WHERE url = $1",
            &[&url]
        ) {
            Ok(t) => t,
            // bad i know
            Err(_t) => return UsedUrlStatus::UrlDoesntExist
        };

        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("").as_secs();
        let expiry_time = used_url.get::<&str, i64>("crawl_again_at");

        if (expiry_time as u64) < now {
            return UsedUrlStatus::CanCrawlUrl;
        }
        return UsedUrlStatus::CannotCrawlUrl;
    }

    fn crawledurls_add(self: &mut Self, url: &str) -> UsedUrlStatus {
        let oneweek = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("").as_secs() + 7*86400;

        let _used_url = match self.client.execute(
            "INSERT INTO crawledurls VALUES ($1, $2)",
            &[&url, &(oneweek as i64)]
        ) {
            Ok(t) => t,
            Err(_) => {return UsedUrlStatus::URLExists}
        };

        return UsedUrlStatus::NewUrl
    }
}
