use postgres::{Client, NoTls};
use crate::crawled_page;

pub struct Database {
    client: Client,
}

impl Database {
    pub fn new() -> Self {
        let newClient = Client::connect("host=localhost user=prushton password=password dbname=maindb", NoTls).unwrap();

        let db: Self = Self{
            client: newClient,
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
                crawled_again_at integer
            );

            CREATE TABLE IF NOT EXISTS IndexedData (
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
            Ok(t) => {return Ok("Done".to_string())},
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
            let lowercaseword = word.to_lowercase();
            match self.client.execute(
                "INSERT INTO crawledwords VALUES ($1, $2, $3)",
                &[&crawledpage.url, &lowercaseword, &(*count as i32)]
            ) {
                Ok(_) => {},
                Err(t) => panic!("Error writing to database: {}", t)
            };
        }
    }
}