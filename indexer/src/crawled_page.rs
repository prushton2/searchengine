use std::collections::HashMap;

use serde::Deserialize;
use serde_json;

use crate::indexed_page;

#[derive(Debug, Deserialize)]
pub struct V1 {
    pub version: u32,
    pub url: String,
    pub words: HashMap<String, u64>
}

impl V1 {
    pub fn from_string(string: &str) -> Result<Self, &str> {
        let object: V1 = match serde_json::from_str(&string) {
            Ok(t) => t,
            Err(_t) => return Err("Error deserializing json")
        };
        if object.version == 1 {
            return Ok(object);
        }
        return Err("Invalid version, expected version 1")
    }

    // pub fn save(self: &mut Self, BASEPATH: &str) {
    //     for (mut word, count) in self.words.clone().into_iter() {
        //         if word.len() < 2 {
            //             continue
            //         }
            //         word = word.to_lowercase();
            //         let second_byte_index = match word.char_indices().nth(2) {
                //             Some(t) => t.0,
                //             None => continue
                //         };
                
                //         let first_two_chars = &word[0..second_byte_index];
                
    //         let path: PathBuf = Path::new(&BASEPATH).join(first_two_chars).join(&word);
            
    //         // println!("{:?}", path.as_os_str());
    //         // let file = 
    //     }
    // }

    pub fn index(self: &Self) -> Result<indexed_page::IndexedPage, &str> {
        let mut page: indexed_page::IndexedPage = indexed_page::IndexedPage{
            url: self.url.clone(),
            words: [].into()
        };

        for (word, count) in self.words.clone().into_iter() {
            page.words.insert(word, count);
        }

        return Ok(page)
        
    }
}
