use std::path::Path;
use std::path::PathBuf;
use std::collections::HashMap;
use std::vec;
use std::fs;
use std::io::Write;

use serde::{Serialize, Deserialize};
use serde_json;

pub struct IndexedPage {
    pub url: String,
    pub words: HashMap<String, u64>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexedWord {
    pub urls: Vec<(String, u64)>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub urls: HashMap<String, SiteMetadata>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SiteMetadata {
    pub title: String
}

impl IndexedPage {
    pub fn write_text(self: &Self, basepath: &str) {
        let mut dirbuilder = fs::DirBuilder::new();
        dirbuilder.recursive(true);
        
        // iterate over each word
        for (mut word, new_score) in self.words.clone().into_iter() {
            if word.len() < 3 || word.len() > 64{
                continue
            }

            // preprocessing (lowercase it, evaluate its path)
            word = word.to_lowercase();
            let second_byte_index = match word.char_indices().nth(2) {
                Some(t) => t.0,
                None => {continue}
            };

            let first_two_chars = &word[0..second_byte_index];
            
            let mut path: PathBuf = Path::new(&basepath).join(first_two_chars).join(&word);
            path.set_extension("json");


            // ensure path exists and get the currently indexed data
            let mut file_contents = match get_indexed_word(&path, &dirbuilder) {
                Ok(t) => t,
                Err(_) => IndexedWord{urls: vec![]}
            };
            
            // update score in contents
            let mut found = false;
            for (index, (link, _old_score)) in file_contents.urls.clone().into_iter().enumerate() {
                // check link, if same update score
                if link == self.url {
                    file_contents.urls[index].1 = new_score;
                    found = true;
                }
            }

            // append to list if the url doesnt exist in the list already
            if !found {
                file_contents.urls.push((self.url.clone(), new_score));
            }

            // write to disk
            let serialized = match serde_json::to_string(&file_contents) {
                Ok(t) => t,
                Err(t) => panic!("Error serializing to file: {:?}", t)
            };

            let mut file = match fs::File::create(&path) {
                Ok(t) => t,
                Err(t) => panic!("Error opening file {:?}", t)
            };

            let result = file.write_all(serialized.as_bytes());

            if result.is_err() {
                panic!("Error writing to file {:?}\n {:?}", path.as_os_str(), result.err());
            }
        }
    }

    pub fn write_metadata(self: &Self, basepath: &str) -> Result<&'static str, &'static str> {
        let path: PathBuf = Path::new("../indexer_data/site_metadata.json");
        let file_contents: Metadata;

        if path.exists() {
            let filestring = match fs::read_to_string(path.as_os_str()) {
                Ok(t) => t,
                Err(_) => {return Err("Unable to open file")}
            };

            file_contents = match serde_json::from_str(&filestring) {
                Ok(t) => t,
                Err(_) => {return Err("Error deserializing file")}
            };  
        } else {
            let dir_result = dirbuilder.create(&path.parent().unwrap());

            if dir_result.is_err() {
                return Err("Error building dirs")
            }

            let result = fs::File::create(&path);

            if result.is_err() {
                return Err("Error creating file");
            }

            file_contents = Metadata{
                urls: [].into()
            }
        }

        file_contents.urls.insert(self.url.clone(), SiteMetadata{
            title: self.title.clone()
        });

        let serialized = match serde_json::to_string(&file_contents) {
            Ok(t) => t,
            Err(_t) => return Err("Error serializing metadata contents")
        };

        let _ = fs::write_all(serialized.as_bytes());

    }
}

fn get_indexed_word(path: &PathBuf, dirbuilder: &fs::DirBuilder) -> Result<IndexedWord, &'static str> {
    let file_contents: IndexedWord;

    if path.exists() {
        // load the existing file
        let filestring = match fs::read_to_string(path.as_os_str()) {
            Ok(t) => t,
            Err(_) => {return Err("Unable to open file")}
        };

        file_contents = match serde_json::from_str(&filestring) {
            Ok(t) => t,
            Err(_) => {return Err("Error deserializing file")}
        };  

    } else {
        // create the new file and init the contents
        let dir_result = dirbuilder.create(&path.parent().unwrap());

        if dir_result.is_err() {
            return Err("Error building dirs")
        }

        let result = fs::File::create(&path);

        if result.is_err() {
            return Err("Error creating file");
        }

        file_contents = IndexedWord{
            urls: vec![]
        };
    }
    return Ok(file_contents);
}