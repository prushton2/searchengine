use std::str;
use std::collections::HashMap;

use serde::Serialize;

use crate::database;
use crate::page_content::PageContent;

#[derive(Debug, Serialize)]
pub struct CrawledPage {
    pub version: u32,
    pub title: String,
    pub url: String,
    pub description: String,
    pub words: HashMap<String, u32>
}

impl CrawledPage {
    pub fn from_page_content(page: &PageContent, url: &str) -> Result<CrawledPage, String> {
        let mut crawled_page = CrawledPage{
            version: 1,
            title: String::from(page.title.clone()),
            url: String::from(url),
            description: String::from(page.description.clone()),
            words: [].into()
        };

        if crawled_page.title.len() > 512 {
            crawled_page.title = database::safe_truncate(&crawled_page.title, 512);
        }
        if crawled_page.url.len() > 512 {
            crawled_page.url = database::safe_truncate(&crawled_page.url, 512);
        }
        if crawled_page.description.len() > 1024 {
            crawled_page.description = database::safe_truncate(&crawled_page.description, 1024);
        }

        let alphanumeric_text = page.text.replacen(|c| {!char::is_alphanumeric(c)}, " ", usize::MAX);
    
        for word in alphanumeric_text.as_str().split(' ') {
            let trimmed = word.trim().chars().filter(|c| c.is_alphanumeric()).collect::<String>().to_lowercase();
    
            if trimmed == "" {
                continue;
            }
    
            crawled_page.words.entry(trimmed).and_modify(|c| *c += 1).or_insert(1);
        }
    
        Ok(crawled_page)
    }
}