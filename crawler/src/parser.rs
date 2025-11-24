// The parser doesnt implement an interface as it doesnt need state. Its job is to take raw bytes, and spit out some data regarding the content
use std::cell::RefCell;
use std::collections::HashMap;
use std::default::Default;
use std::ops::{Deref, DerefMut};

use regex::Regex;

use html5ever::interface::QualName;
use html5ever::tendril::*;
use html5ever::tokenizer::BufferQueue;
use html5ever::tokenizer::Token::CharacterTokens;
use html5ever::tokenizer::{EndTag, StartTag, TagToken};
use html5ever::tokenizer::{Token, TokenSink, TokenSinkResult, Tokenizer, TokenizerOpts};
use html5ever::{LocalName, ns};

#[derive(Clone)]
pub struct ParsedData {
    pub description: String,
    pub title: String,
    pub words: Vec<Word>,
    pub urls: Vec<String>,
}

#[derive(Clone)]
pub struct Word {
    pub word: String,
    pub parent: String,
    pub count: i32,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ParseHTMLError {
    TextDecodeFailed,
}

struct TokenSinkState {
    pub parent: Vec<String>,
    pub parsed_data: ParsedData,
    pub words: HashMap<(String, String), i32>,
}

struct TokenSinkWrapper {
    pub rc: RefCell<TokenSinkState>,
}

impl TokenSink for TokenSinkWrapper {
    type Handle = ();

    fn process_token(&self, token: Token, _line_number: u64) -> TokenSinkResult<()> {
        let link_name = QualName::new(None, ns!(), LocalName::from("href"));

        let mut binding = self.rc.borrow_mut();
        let state: &mut TokenSinkState = binding.deref_mut();

        match token {
            TagToken(tag) if tag.kind == StartTag => {
                state.parent.push(tag.name.as_ref().to_string());
                if tag.name.to_string() == "a" {
                    let attrs = tag.attrs;
                    for attr in attrs {
                        if attr.name == link_name {
                            state.parsed_data.urls.push(attr.value.to_string());
                        }
                    }
                }
            }
            TagToken(tag) if tag.kind == EndTag => {
                state.parent.pop();
            }
            CharacterTokens(tendril) if state.parent.contains(&String::from("body")) => {
                // append to description
                if state.parsed_data.description.chars().count() < 512 {
                    state.parsed_data.description.push(' ');
                    
                    let new_desc =
                        safe_truncate(&(*tendril).to_string(), 512 - state.parsed_data.description.chars().count());

                    state.parsed_data.description.push_str(&new_desc);
                }

                // get parent
                let parent = match state.parent.last() {
                    Some(n) => n,
                    None => &String::from(""),
                };

                let words = tendril.deref().split(" ");

                for word in words {
                    let clean_word = clean_alphanumeric(word);

                    if clean_word.len() == 0 {
                        continue
                    }

                    match state.words.insert((clean_word.clone(), parent.clone()), 1) {
                        Some(n) => {
                            state
                                .words
                                .insert((clean_word.clone(), parent.clone()), n + 1);
                        }
                        None => {}
                    }
                }
            }

            CharacterTokens(tendril) if state.parent.contains(&String::from("head")) => {
                // get parent
                let parent = match state.parent.last() {
                    Some(n) => n,
                    None => &String::from(""),
                };

                // set title
                if parent == "title" {
                    state.parsed_data.title = (*tendril).to_string();
                }
            }

            _ => {}
        }
        TokenSinkResult::Continue
    }
}

pub fn parse_html(content: Vec<u8>, _url: &String) -> Result<ParsedData, ParseHTMLError> {
    let sink: RefCell<TokenSinkState> = RefCell::new(TokenSinkState {
        parent: vec!["".to_string()],
        words: [].into(),
        parsed_data: ParsedData {
            description: String::from(""),
            title: String::from(""),
            words: vec![],
            urls: vec![],
        },
    });

    let mut input = BufferQueue::default();

    // Convert Vec<u8> to ByteTendril and push to input
    let tendril = ByteTendril::from_slice(&content);
    let try_reinterpret = match tendril.try_reinterpret::<fmt::UTF8>() {
        Ok(t) => t,
        Err(_) => return Err(ParseHTMLError::TextDecodeFailed),
    };
    input.push_back(try_reinterpret);

    let tok = Tokenizer::new(TokenSinkWrapper { rc: sink }, TokenizerOpts::default());
    let _ = tok.feed(&mut input);
    tok.end();

    let sink_state = tok.sink.rc.into_inner(); // Use into_inner() to take ownership
    let mut parsed_data = sink_state.parsed_data;

    for ((word, parent), count) in sink_state.words {
        let o = Word{
            word: word,
            parent: parent,
            count: count
        };
        parsed_data.words.push(o);
    }

    parsed_data.description = clean_description(&parsed_data.description);

    return Ok(parsed_data);
}

fn clean_description(text: &str) -> String {
    let remove_non_alphanumeric = Regex::new(r"(^ )|[^a-zA-Z\d ]|[\r\n]").expect("clean_description regex did not compile");
    let cleaned = remove_non_alphanumeric.replace_all(&text, "").to_string();
    return cleaned;
}

pub fn safe_truncate(string: &String, count: usize) -> String {
    return string.chars().take(count).collect();
}

fn clean_alphanumeric(text: &str) -> String {
    let remove_non_alphanumeric = Regex::new(r"[^a-zA-Z\d]").expect("clean_alphanumeric regex did not compile");
    let cleaned = remove_non_alphanumeric
        .replace_all(&text, "")
        .to_lowercase();
    return cleaned;
}
