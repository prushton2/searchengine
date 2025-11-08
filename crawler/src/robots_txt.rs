use robotstxt::DefaultMatcher;
use url::Url;
use std::str;

use crate::http_request;
// Handles interfacing with robots.txt
    
pub trait RobotsTXT {
    fn allows_url(&self, url: &str) -> bool;
    fn fetch_new_robots_txt(&mut self, url: &str) -> Result<String, String>;
}

pub struct RobotsTXTCrate {
    content: String,
    request_object: http_request::HTTPRequest,
}

impl RobotsTXT for RobotsTXTCrate {
    fn allows_url(&self, url: &str) -> bool {
        let mut matcher = DefaultMatcher::default();
        return matcher.one_agent_allowed_by_robots(&self.content, &self.request_object.get_user_agent(), url)
    }

    fn fetch_new_robots_txt(&mut self, url: &str) -> Result<String, String> {
        return match Url::parse(url) {
            Ok(t) => {
                self.content = self.fetch_robots_txt(&t); 
                Ok(String::from("Ok"))
            }
            Err(t) => {
                self.content = String::from("");
                Err(t.to_string())
            }
        }
    }
}

impl RobotsTXTCrate {
    pub fn new(request_object: http_request::HTTPRequest) -> Self {
        return RobotsTXTCrate{
            content: String::from(""),
            request_object: request_object,
        }
    }
    
    fn fetch_robots_txt(&self, url_object: &url::Url) -> String {
        let mut robots_path = url_object.clone();
        robots_path.set_path("/robots.txt");
        robots_path.set_query(None);
        robots_path.set_fragment(None);
        
        let robots_bytes: Vec<u8> = match self.request_object.request(robots_path.as_str()) {
            Ok(t) => t.0,
            Err(_) => "user-agent: *\ndisallow:".as_bytes().to_owned()
        };
        
        return match str::from_utf8(&robots_bytes) {
            Ok(t) => t.to_string(),
            Err(_) => "user-agent: *\ndisallow:".into(),
        };
    }
}