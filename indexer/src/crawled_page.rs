use std::path::Path;
use std::path::PathBuf;
use std::collections::HashMap;

use serde::Deserialize;
use serde_json;

#[derive(Debug, Deserialize)]
pub struct CrawledPage {
    pub version: u32,
    pub url: String,
    pub words: HashMap<String, u32>
}

impl CrawledPage {
    pub fn from_string(string: &str) -> Result<Self, &str> {
        return match serde_json::from_str(&string) {
            Ok(t) => Ok(t),
            Err(t) => Err("Error deserializing json")
        };
    }

    pub fn save(self: &mut Self, BASEPATH: &str) {
        for (mut word, count) in self.words.clone().into_iter() {
            if word.len() < 2 {
                continue
            }
            word = word.to_lowercase();
            let second_byte_index = match word.char_indices().nth(2) {
                Some(t) => t.0,
                None => continue
            };
    
            let first_two_chars = &word[0..second_byte_index];
    
            let path: PathBuf = Path::new(&BASEPATH).join(first_two_chars).join(&word);
            
            // println!("{:?}", path.as_os_str());
            // let file = 
        }
    }
}
