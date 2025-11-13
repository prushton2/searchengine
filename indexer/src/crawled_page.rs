
#[derive(Debug)]
pub struct CrawledPage {
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

