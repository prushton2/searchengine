use std::str;
use std::collections::HashMap;

use url::Url;
use serde::Serialize;
use scraper::{Html, Selector};

#[derive(Clone)]
struct PageContent {
    links: Vec<String>,
    text: String
}

#[derive(Debug, Serialize)]
pub struct CrawledPage {
    version: u32,
    url: String,
    words: HashMap<String, u32>
}

pub fn crawl_page(bytes: &[u8], url: &str) -> Result<CrawledPage, &'static str> {
    
    let mut page_content = match strip_html(bytes) {
        Ok(t) => t,
        Err(t) => return Err(t)
    };

    let url_host = get_host_from_url(&url).unwrap();
    
    for (index, link) in page_content.links.clone().into_iter().enumerate() {
        // resolve relative urls
        if link.chars().nth(0) == Some('/') {
            let string = ["https://", url_host, link.as_str()].concat(); //todo: fix this
            page_content.links[index] = string;
        }
    }

    // crawled_urls.extend(page_content.links.clone());

    let crawled_page = create_crawled_page_object(&page_content, url).unwrap();

    // let _ = write_to_file(&crawled_page.unwrap());

    return Ok(crawled_page);
}

fn strip_html(bytes: &[u8]) -> Result<PageContent, &'static str> {
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

fn create_crawled_page_object(page: &PageContent, url: &str) -> Result<CrawledPage, String> {
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

fn get_host_from_url(url: &str) -> Result<&'static str, &'static str> {
    let parsed = match Url::parse(url) {
        Ok(t) => t,
        Err(_t) => return Err("Error parsing url string. Is it valid?")
    };

    return match parsed.host_str() {
        Some(t) => Ok(t),
        None => Err("No hostname in url")
    };
}