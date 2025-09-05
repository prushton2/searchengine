use std::str;
use std::collections::HashMap;
use std::time::SystemTime;
use std::fs;
use std::io::Write;
use curl::easy::WriteError;
use scraper::{Html, Selector};
use serde::Serialize;
use serde_json;

#[derive(Clone)]
struct PageContent {
    links: Vec<String>,
    text: String
}

#[derive(Debug, Serialize)]
struct CrawledPage {
    version: u32,
    url: String,
    words: HashMap<String, u32>
}

pub fn process_out(bytes: &[u8], url: &str, crawled_urls: &mut Vec<String>) -> Result<usize, WriteError> {
    
    let mut page_content = strip_html(bytes).unwrap();

    // resolve relative urls
    let mut root_url_iter = url.split("/");

    let root_url: String = [root_url_iter.next().unwrap(), "//", root_url_iter.next().unwrap(), root_url_iter.next().unwrap()].concat();

    for (index, link) in page_content.links.clone().into_iter().enumerate() {
        if link.chars().nth(0) == Some('/') {
            let string = [root_url.as_str(), link.as_str()].concat();
            page_content.links[index] = string;
        }
    }

    crawled_urls.extend(page_content.links.clone());

    let crawled_page = crawl_page(&page_content, url);

    let _ = write_to_file(&crawled_page.unwrap());

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

fn crawl_page(page: &PageContent, url: &str) -> Result<CrawledPage, String> {
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

fn write_to_file(crawled_page: &CrawledPage) -> Result<&'static str, &'static str> {
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("").as_secs();
    let filename = ["../crawler_data/output/", now.to_string().as_str(), ".json"].concat();

    let file_result = fs::File::create(filename);
    let serialized = serde_json::to_string(&crawled_page);

    if serialized.is_err() {
        return Err("Serialization Failed");
    }

    if file_result.is_err() {
        return Err("Error opening file");
    }

    let mut file = file_result.unwrap();

    let _ = file.write_all(serialized.unwrap().as_bytes());
    return Ok("File Written")
}