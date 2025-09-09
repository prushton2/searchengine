use std::str;
use std::collections::HashMap;

use serde::Serialize;

use crate::page_content::PageContent;

#[derive(Debug, Serialize)]
pub struct CrawledPage {
    pub version: u32,
    pub url: String,
    pub words: HashMap<String, u32>
}
impl CrawledPage {
    pub fn from_page_content(page: &PageContent, url: &str) -> Result<CrawledPage, String> {
        let mut crawled_page = CrawledPage{
            version: 1,
            url: String::from(url),
            words: [].into()
        };
    
        for word in page.text.as_str().split(' ') {
            let trimmed = word.trim().chars().filter(|c| c.is_alphanumeric()).collect::<String>();
    
            if trimmed == "" {
                continue;
            }
    
            crawled_page.words.entry(trimmed).and_modify(|c| *c += 1).or_insert(1);
        }
    
        Ok(crawled_page)
    }
}