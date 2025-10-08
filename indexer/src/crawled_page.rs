use std::collections::HashMap;
use url::Url;

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

        // cloning this lets me pass ownership to page and consumes the clone
        for (word, count) in self.words.clone().into_iter() {
            page.words.insert(word.to_lowercase(), count);
        }

        let parsed_url = Url::parse(&self.url).unwrap();

        // some extra postprocessing can be done to ensure we deal with .ext files

        // add the domain components so you can search 'google' and get google.com
        let binding = parsed_url.host().unwrap().to_string();
        let mut split_domain = binding.split('.');
        
        // this is the tld. pop it off
        let _ = split_domain.next_back(); 

        //this is the domain name, give it a higher score
        match split_domain.next_back() { 
            Some(t) => {
                page.words.insert(t.to_string().to_lowercase(), 20);
            },
            None => {}
        }
        
        // all subdomains, give a high score
        for domain_string in split_domain { 
            page.words.insert(domain_string.to_string().to_lowercase(), 10);
        }

        // give each path segment a low score
        for path_component in Self::filter_text(parsed_url.path()).split(' ') {
            if path_component == "" {
                continue;
            }
            page.words.insert(path_component.to_string().to_lowercase(), 5);
        }

        return Ok(page)
    }

    fn filter_text(string: &str) -> String {
        return string.replacen(|c| {!char::is_alphanumeric(c)}, " ", usize::MAX)
    }

}
