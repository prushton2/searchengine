use std::collections::HashMap;

pub struct IndexedPage {
    pub url: String,
    pub title: String,
    pub description: String,
    pub words: HashMap<String, u64>
}