use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Add;

use crate::crawled_page;
use crate::database;
use crate::dictionary;

pub trait IndexedPage {
    fn from_crawled_page(self: &mut Self, page: crawled_page::CrawledPage, dict: &dyn dictionary::Dictionary);
    fn consume_into_db(self: &mut Self, db: &mut dyn database::Database) -> Result<(), database::Error>;
}

pub struct BasicIndexedPage {
    pub url: String,
    pub title: String,
    pub description: String,
    pub words: HashMap<String, u64>
}

impl IndexedPage for BasicIndexedPage {
    fn from_crawled_page(self: &mut Self, page: crawled_page::CrawledPage, dict: &dyn dictionary::Dictionary) {
        self.url = page.url;
        self.title = page.title;
        self.description = page.description;

        for word in page.words {
            if dict.get_word_status(&word.word) == dictionary::WordType::StopWord {
                continue
            }

            let multiplier = match word.parent.as_str() {
                "title" => 30,
                "h1" => 20,
                "h2" => 18,
                "h3" => 16,
                "h4" => 14,
                "h5" => 12,
                "h6" => 10,
                "a" => 5,
                _ => 1
            };

            self.words.insert_or_sum(word.word, (word.count * multiplier) as u64);
        }

        for (word, score) in self.words.clone().iter() {
            if *score <= 2 {
                self.words.remove(word);
            }
        }
    }

    fn consume_into_db(self: &mut Self, db: &mut dyn database::Database) -> Result<(), database::Error> {
        match db.write_indexed_page(&self.url, &self.title, &self.description) {
            Ok(_) => {}
            Err(t) => return Err(t)
        };
        match db.write_indexed_words(&self.url, &mut self.words.clone().into_iter()) {
            Ok(_) => return Ok(()),
            Err(t) => return Err(t)
        };
    }
}

impl BasicIndexedPage {
    pub fn new() -> Self {
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