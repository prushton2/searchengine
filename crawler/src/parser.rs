use scraper::{Html, Selector, ElementRef, Element};
use std::str;
use regex::Regex;

pub struct ParsedData {
    words: Vec<Word>,
    urls: Vec<String>
}

pub struct Word {
    word: String,
    parent: String,
    count: i32
}

#[derive(Debug)]
pub enum ParseHTMLError {
    TextDecodeFailed
}

pub fn parse_html(content: Vec<u8>, url: String) -> Result<ParsedData, ParseHTMLError> {
    let parsedData = ParsedData {
        words: vec![],
        urls: vec![]
    };

    let html: &str = match str::from_utf8(&content) {
        Ok(v) => v,
        Err(_) => return Err(ParseHTMLError::TextDecodeFailed)
    };

    let document = Html::parse_document(html);

    let title_selector = Selector::parse("title").unwrap();
    let link_selector = Selector::parse("a").unwrap();
    let body_selector = Selector::parse("body").unwrap();


    for element in document.select(&body_selector) {
        for child in element.descendent_elements() {
            if remove_child_html(&child.inner_html()).len() != 0 {
                println!("{}: {}\n", child.value().name(), remove_child_html(&child.inner_html()));
            }
        }
    }

    return Ok(parsedData)

}

fn remove_child_html(html: &String) -> String {
    // Remove all tags and content
    let remove_tags_re = Regex::new(r"<[^>]*>[\s\S]*?</[^>]*>").unwrap();
    let cleaned = remove_tags_re.replace_all(html, "");
    let trimmed = cleaned.trim();

    // Remove any remaining tags (<br>, <img>, etc..)

    if trimmed.chars().nth(0) == Some('<') && trimmed.chars().last() == Some('>') {
        return String::from("");
    }
    
    let remove_tags_re = Regex::new(r"<[^>]+>").unwrap();
    let cleaned = remove_tags_re.replace_all(html, "");
    let trimmed = cleaned.trim();

    return trimmed.to_owned();
}