use std::str;

use scraper::{Html, Selector};

#[derive(Clone)]
pub struct PageContent {
    pub links: Vec<String>,
    pub title: String,
    pub text: String
}

impl PageContent {
    pub fn from_html(bytes: &[u8]) -> Result<PageContent, &'static str> {
        let mut pagecontent: PageContent = PageContent { links: vec![], text: String::from(""), title: String::from("") };

        let html: &str = match str::from_utf8(bytes) {
            Ok(v) => v,
            Err(_e) => return Err("Could not decode text from bytes")
        };

        let document = Html::parse_document(html);

        let title_selector = Selector::parse("title").unwrap();
        let link_selector = Selector::parse("a").unwrap();
        let body_selector = Selector::parse("body").unwrap();

        // body text parsing
        for element in document.select(&body_selector) {
            for child in element.text() {
                pagecontent.text = pagecontent.text + " " + child;
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

            pagecontent.links.push(String::from(link))
        }

        for element in document.select(&title_selector) {
            if pagecontent.title == "" {
                pagecontent.title = element.text().nth(0).unwrap().to_string()
            }
        }

        Ok(pagecontent)
    }
}