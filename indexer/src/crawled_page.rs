
#[derive(Debug)]
pub struct Crawled_page {
    pub url: String,
    pub description: String,
    pub title: String,
    pub words: Vec<Word>
}

#[derive(Debug)]
pub struct Word {
    pub word: String,
    pub parent: String,
    pub count: i32
}

