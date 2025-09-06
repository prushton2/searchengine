use std::str;
use std::collections::HashMap;

use serde::Serialize;
use scraper::{Html, Selector};

#[derive(Clone)]
pub struct PageContent {
    pub links: Vec<String>,
    pub text: String
}

#[derive(Debug, Serialize)]
pub struct CrawledPage {
    pub version: u32,
    pub url: String,
    pub words: HashMap<String, u32>
}

pub fn strip_html(bytes: &[u8]) -> Result<PageContent, &'static str> {
    let mut page_content: PageContent = PageContent { links: vec![], text: String::from("") };

    let html: &str = match str::from_utf8(bytes) {
        Ok(v) => v,
        Err(_e) => return Err("Could not decode text from bytes")
    };

    let document = Html::parse_document(html);

    let link_selector = Selector::parse("a").unwrap();
    let body_selector = Selector::parse("body").unwrap();

    // body text parsing
    for element in document.select(&body_selector) {
        for child in element.text() {
            page_content.text = page_content.text + " " + child;
        }
    }

    for element in document.select(&link_selector) {
        let link = match element.attr("href") {
            Some(url) => url,
            None => ""
        };

        if link == "" {
            continue;
        }

        page_content.links.push(String::from(link))
    }
    Ok(page_content)
}

pub fn create_crawled_page_object(page: &PageContent, url: &str) -> Result<CrawledPage, String> {
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