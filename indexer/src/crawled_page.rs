use std::collections::HashMap;

use serde::Deserialize;
use serde_json;

use crate::indexed_page;

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

    pub fn index(self: &Self) -> Result<indexed_page::IndexedPage, &str> {
        let mut page: indexed_page::IndexedPage = indexed_page::IndexedPage{
            url: self.url.clone(),
            words: [].into()
        };

        for (word, count) in self.words.clone().into_iter() {
            page.words.insert(word, count);
        }

        return Ok(page)
        
    }
}
