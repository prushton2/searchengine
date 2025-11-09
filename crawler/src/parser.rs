// The parser doesnt implement an interface as it doesnt need state. Its job is to take raw bytes, and spit out some data regarding the content

use scraper::{Html, Selector};
use std::str;
use regex::Regex;
use std::collections::HashMap;

pub struct ParsedData {
    pub description: String,
    pub title: String,
    pub words: Vec<Word>,
    pub urls: Vec<String>
}

pub struct Word {
    pub word: String,
    pub parent: String,
    pub count: i32
}

#[derive(Debug)]
pub enum ParseHTMLError {
    TextDecodeFailed
}

pub fn parse_html(content: Vec<u8>, _url: &String) -> Result<ParsedData, ParseHTMLError> {
    let mut parsed_data = ParsedData {
        title: "".to_string(),
        description: "".to_string(),
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

    let mut wordmap: HashMap<(String, String), i32> = [].into();

    for element in document.select(&body_selector) {
        for child in element.descendent_elements() {
            let parent_node = child.value().name();
            let child_text = remove_child_html(&child.inner_html());

            if child_text.len() == 0 { continue };
            let cleaned = clean_text(&child_text);
            let parts = cleaned.split(" ");
            
            for word in parts {
                if word.len() == 0 { continue; }

                match wordmap.insert((parent_node.to_string(), word.to_string()), 1) {
                    Some(n) => {wordmap.insert((parent_node.to_string(), word.to_string()), n+1);},
                    None => {}
                }
            }
        }
    }

    for element in document.select(&body_selector) {
        let text = element.text().collect::<Vec<_>>().join(" ");
        if !text.is_empty() {
            parsed_data.description = text.chars().take(512).collect::<String>();
            break;
        }
    }

    for element in document.select(&title_selector) {
        for text in element.text() {
            parsed_data.title = text.to_string();
            if text.len() == 0 { continue };
            let cleaned = clean_text(&text);
            let parts = cleaned.split(" ");

            for word in parts {
                if word.len() == 0 {
                    continue;
                }

                match wordmap.insert(("title".to_string(), word.to_string()), 1) {
                    Some(n) => {wordmap.insert(("title".to_string(), word.to_string()), n+1);},
                    None => {}
                }
            }
        }
    }

    for element in document.select(&link_selector) {
        let link = match element.attr("href") {
            Some(url) => url,
            None => ""
        };
        if link == "" { continue; }

        parsed_data.urls.push(String::from(link))
    }

    for ((parent, word), count) in wordmap.iter() {
        parsed_data.words.push(
            Word{
                word: word.clone(),
                parent: parent.clone(),
                count: *count
            }
        );
    }

    return Ok(parsed_data)
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

fn clean_text(text: &str) -> String {
    let remove_non_alphanumeric = Regex::new(r"[^a-zA-Z\d\s:]|[\r\n]").unwrap();
    let cleaned = remove_non_alphanumeric.replace_all(&text, " ").to_lowercase();
    return cleaned;
}