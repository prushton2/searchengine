use std::str;
use curl::easy::WriteError;
use scraper::{Html, Selector};
use std::collections::HashMap;


#[derive(Clone)]
struct PageContent {
    links: Vec<String>,
    text: String
}

#[derive(Debug)]
struct CrawledPage {
    version: u32,
    url: String,
    words: HashMap<String, u32>
}

pub fn process_out(bytes: &[u8], url: &str, crawled_urls: &mut Vec<String>) -> Result<usize, WriteError> {
    
    let page_content = strip_html(bytes).unwrap();

    crawled_urls.extend(page_content.links.clone());

    let crawled_page = crawl_page(&page_content, url);

    println!("{:?}", crawled_page);
    
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
            none => ""
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