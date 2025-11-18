use postgres::{Client, NoTls, error::SqlState};
use crate::config::PostgresDBInfo;
use crate::crawled_page;

pub trait Database {
    fn get_crawled_page(self: &mut Self) -> Option<crawled_page::CrawledPage>;
    fn crawled_page_len(self: &mut Self) -> u32;
    fn write_indexed_page(self: &mut Self, url: &str, title: &str, desc: &str) -> Result<(), Error>;
    fn write_indexed_words(self: &mut Self, url: &str, words: &mut dyn Iterator<Item = (String, u64)>) -> Result<(), Error>;
}

#[allow(dead_code)]
#[derive(Debug)]
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

    fn get_crawled_page(self: &mut Self) -> Option<crawled_page::CrawledPage> {
        let response = match self.client.query(
            "SELECT * FROM (SELECT * FROM crawleddata LIMIT 1) AS sq LEFT JOIN crawledwords ON sq.url=crawledwords.url",
            &[]
        ) {
            Ok(t) => t,
            Err(_) => return None
        };

        let mut crawled_data = crawled_page::CrawledPage {
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

        match self.client.execute(
            "DELETE FROM crawledurls WHERE url=$1;",
            &[&crawled_data.url]
        ) {
            Ok(_) => {}
            Err(t) => println!("error: {:?}", t)
        }

        match self.client.execute(
            "DELETE FROM crawleddata WHERE url=$1;",
            &[&crawled_data.url]
        ) {
            Ok(_) => {}
            Err(t) => println!("error: {:?}", t)
        }

        return Some(crawled_data);
    }

    fn write_indexed_page(self: &mut Self, url: &str, title: &str, desc: &str) -> Result<(), Error> {
        match self.client.query(
            "INSERT INTO sitemetadata VALUES ($1, $2, $3) 
                ON CONFLICT (url)
                DO UPDATE SET
                    title = $2,
                    description = $3",
            &[&url, &title, &desc]
        ) {
            Ok(_) => return Ok(()),
            Err(t) => return Err(Error::SQLError(t.code().cloned()))
        };
    }

    fn write_indexed_words(self: &mut Self, url: &str, word_iterator: &mut dyn Iterator<Item = (String, u64)>) -> Result<(), Error> {
        let mut words: Vec<String> = vec![];
        let mut weights: Vec<i32> = vec![];
        let mut urls: Vec<&str> = vec![];
        
        for (word, value) in word_iterator {
            words.push(word);
            weights.push(value as i32);
            urls.push(url);
        }

        match self.client.execute(
            "INSERT INTO indexedwords (url, word, weight)
            SELECT * FROM UNNEST($1::text[], $2::text[], $3::int[]);",
            &[&urls, &words, &weights]
        ) {
            Ok(_) => return Ok(()),
            Err(t) => return Err(Error::SQLError(t.code().cloned()))
        }
    }
}