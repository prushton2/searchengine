use serde::{Serialize, Deserialize};
use log::LevelFilter;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub crawler: CrawlerConfig,
    pub database: PostgresDBInfo
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PostgresDBInfo {
    pub host: String,
    pub username: String,
    pub password: String,
    pub dbname: String
}

#[derive(Serialize, Deserialize)]
pub struct CrawlerConfig {
    pub crawler_threads: i32,
    pub max_crawl_depth: i32,
    pub user_agent: String,
    pub seed_url: String,
    pub log: String
}

impl Config {
    pub fn read_from_file(filename: &str) -> Self {
        let contents = std::fs::read_to_string(filename)
            .expect("Failed to read config file");
        serde_yaml::from_str(&contents)
            .expect("Failed to parse config file")
    }
}

pub fn parse_log_level(level_str: &str) -> LevelFilter {
    match level_str.to_lowercase().as_str() {
        "off" => LevelFilter::Off,
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        _ => {
            eprintln!("Unknown log level '{}', defaulting to Info", level_str);
            LevelFilter::Info
        }
    }
}