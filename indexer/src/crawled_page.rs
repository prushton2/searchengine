use std::collections::HashMap;

use serde::Deserialize;
use serde_json;

use crate::indexed_page;
use crate::dictionary;

#[derive(Debug, Deserialize)]
pub struct V1 {
    pub version: u32,
    pub title: String,
    pub url: String,
    pub words: HashMap<String, u64>
}

impl V1 {
    pub fn from_string(string: &str) -> Result<Self, &str> {
        let object: V1 = match serde_json::from_str(&string) {
            Ok(t) => t,
            Err(_t) => return Err("Error deserializing json")
        };
        if object.version == 1 {
            return Ok(object);
        }
        return Err("Invalid version, expected version 1")
    }

    pub fn filter_stop_words(self: &mut Self) -> Result<&'static str, &'static str> {
        for (word, _count) in self.words.clone().into_iter() {
        
            match dictionary::DICTIONARY.get(&word as &str) {
                Some(_) => {
                    self.words.remove(&word);
                }
                None => {}
            }
        }

        return Ok("")
    }

    pub fn index(self: &Self) -> Result<indexed_page::IndexedPage, &str> {
        let mut page: indexed_page::IndexedPage = indexed_page::IndexedPage{
            url: self.url.clone(),
            words: [].into(),
            title: self.title.clone()
        };

        for (word, count) in self.words.clone().into_iter() {
            page.words.insert(word, count);
        }

        return Ok(page)
        
    }

}
