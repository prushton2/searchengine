use std::collections::HashMap;
use url::Url;
use std::hash::Hash;
use std::ops::Add;

use crate::indexed_page;
use crate::dictionary;

pub struct CrawledPage {
    pub title: String,
    pub url: String,
    pub description: String,
    pub words: HashMap<String, u64>
}

impl CrawledPage {
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
            title: self.title.clone(),
            description: self.description.clone(),
            words: [].into()
        };

        // give each word in the body a score in the indexed data
        // cloning this lets me pass ownership to page and consumes the clone
        for (word, count) in self.words.clone().into_iter() {
            let score = (count.ilog2()+1) as u64;
            if score > 2 {
                page.words.insert_or_sum(word, score as u64);
            }
        }
        
        let parsed_url = Url::parse(&self.url).unwrap();
        // add the domain components so you can search 'google' and get google.com
        let binding = parsed_url.host().unwrap().to_string();
        let mut split_domain = binding.split('.');
        
        // this is the tld. give it a score so you can search "iana.org"
        match split_domain.next_back() { 
            Some(t) => {
                page.words.insert_or_sum(t.to_string().to_lowercase(), 15);
            },
            None => {}
        }

        //this is the domain name, give it a higher score
        match split_domain.next_back() { 
            Some(t) => {
                page.words.insert_or_sum(t.to_string().to_lowercase(), 20);
            },
            None => {}
        }
        
        // all subdomains, give a high score
        for domain_string in split_domain { 
            page.words.insert_or_sum(domain_string.to_string().to_lowercase(), 10);
        }

        // give each path segment a low score
        for path_component in Self::filter_text(parsed_url.path()).split(' ') {
            if path_component == "" {
                continue;
            }
            page.words.insert_or_sum(path_component.to_string().to_lowercase(), 5);
        }

        return Ok(page)
    }

    fn filter_text(string: &str) -> String {
        return string.replacen(|c| {!char::is_alphanumeric(c)}, " ", usize::MAX)
    }

}

pub trait MyTrait<K,V> { fn insert_or_sum(&mut self, key: K, val: V); }

impl<K,V> MyTrait<K,V> for HashMap<K,V> 
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