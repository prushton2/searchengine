use std::collections::HashMap;
use url::Url;

use serde::Deserialize;
use serde_json;

use crate::indexed_page;
use crate::dictionary;

#[derive(Debug, Deserialize)]
pub struct CrawledPage {
    pub title: String,
    pub url: String,
    pub description: String,
    pub words: HashMap<String, u64>
}

impl CrawledPage {
    pub fn from_string(string: &str) -> Result<Self, &str> {
        let object: CrawledPage = match serde_json::from_str(&string) {
            Ok(t) => t,
            Err(_t) => return Err("Error deserializing json")
        };
        return Ok(object);
    }

    pub fn filter_stop_words(self: &mut Self) -> Result<&'static str, &'static str> {

        // cloning ensures i dont write to the active map, maybe theres something cheaper i can do
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
            description: self.description.clone(),
            words: [].into(),
            title: self.title.clone()
        };

        // cloning this lets me pass ownership to page and consumes the clone
        for (word, count) in self.words.clone().into_iter() {
            page.words.insert(word, count);
        }

        let parsed_url = Url::parse(&self.url).unwrap();

        // add the domain components so you can search 'google' and get google.com
        for domain_string in parsed_url.host().unwrap().to_string().split('.') {
            page.words.insert(domain_string.to_string(), 10);
        }

        return Ok(page)
        
    }

}
