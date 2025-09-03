use std::str;
use std::collections::HashMap;
use curl::easy::{Easy, WriteError};
use scraper::{Html, Selector};

#[derive(Clone)]
struct PageContent {
    links: Vec<String>,
    text: String
}

struct CrawledPage {
    version: String,
    url: String,
    words: HashMap<String, u32>
}

fn main() {

    let url = "https://example.com/";

    let mut curl = Easy::new();
    curl.url(url).unwrap();
    let _ = curl.write_function(|bytes| {process_out(bytes, url)}).unwrap();
    curl.perform().unwrap();
}

fn process_out(bytes: &[u8], url: &str) -> Result<usize, WriteError> {
    
    let page_content = strip_html(bytes).unwrap();


    
    return Ok(bytes.len());
}

fn strip_html(bytes: &[u8]) -> Result<PageContent, &str> {
    let mut page_content: PageContent = PageContent { links: vec![], text: String::from("") };

    let html: &str = match str::from_utf8(bytes) {
        Ok(v) => v,
        Err(_e) => panic!("Not valid utf8")
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