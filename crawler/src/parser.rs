// use scraper::{Html, Selector};
// use std::str;

pub struct ParsedData {
    words: Vec<Word>,
    urls: Vec<String>
}

pub struct Word {
    word: String,
    parent: String,
    count: i32
}

pub fn parse_html(content: Vec<u8>, url: String) -> ParsedData {
    return ParsedData {
        words: vec![],
        urls: vec![]
    }
}