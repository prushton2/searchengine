use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Add;
use crate::crawled_page;
use crate::database;

pub trait IndexedPage {
    fn from_crawled_page(self: &mut Self, page: crawled_page::Crawled_page);
    fn consume_into_db(self: &mut Self, db: &mut dyn database::Database) -> Result<(), database::Error>;
}

pub struct BasicIndexedPage {
    pub url: String,
    pub title: String,
    pub description: String,
    pub words: HashMap<String, u64>
}

impl IndexedPage for BasicIndexedPage {
    fn from_crawled_page(self: &mut Self, page: crawled_page::Crawled_page) {
        self.url = page.url;
        self.title = page.title;
        self.description = page.description;

        for word in page.words {
            self.words.insert_or_sum(word.word, word.count as u64);
        }
    }

    fn consume_into_db(self: &mut Self, db: &mut dyn database::Database) -> Result<(), database::Error> {
        match db.write_indexed_page(&self.url, &self.title, &self.description) {
            Ok(_) => {}
            Err(t) => return Err(t)
        };
        return Ok(());
        match db.write_indexed_words(&self.url, &mut self.words.into_iter()) {
            Ok(_) => return Ok(()),
            Err(t) => return Err(t)
        };
    }
}

impl BasicIndexedPage {
    fn new() -> Self {
        return BasicIndexedPage {
            url: String::from(""),
            title: String::from(""),
            description: String::from(""),
            words: [].into()
        };
    }
}

pub trait InsertOrSum<K,V> { fn insert_or_sum(&mut self, key: K, val: V); }

impl<K,V> InsertOrSum<K,V> for HashMap<K,V> 
where
    K: Eq + Hash + Clone,
    V: Add<Output = V> + Clone
{
    fn insert_or_sum(&mut self, key: K, val: V) {
        match self.insert(key.clone(), val.clone()) {
            Some(n) => {
                self.insert(key, n+val);
            },
            None => {}
        };
    }
}