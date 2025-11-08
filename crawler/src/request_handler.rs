use crate::robots_txt;
use crate::http_request;

use url::Url;

pub trait RequestHandler<'a, 'b> {
    fn fetch(&mut self, url: &str) -> Result<(Vec<u8>, String), RequestHandlerError>;
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum RequestHandlerError {
    BadURL,
    DisallowedByRobotsTxt,
    HTTPRequestError(http_request::HTTPRequestError)
}


pub struct SimpleRequestHandler<'a, 'b> {
    robotstxt: &'a mut dyn robots_txt::RobotsTXT,
    http_request: &'b http_request::HTTPRequest,
    current_domain: String
}

impl<'a, 'b> RequestHandler<'a, 'b> for SimpleRequestHandler<'a, 'b> {
    fn fetch(&mut self, url: &str) -> Result<(Vec<u8>, String), RequestHandlerError> {
        let mut url_object = match Url::parse(url) {
            Ok(t) => t,
            Err(_) => return Err(RequestHandlerError::BadURL)
        };
        Self::filter_url(&mut url_object);

        if self.current_domain != Self::convert_url_to_domain(&url_object) {
            // try 3 times. if we failed all 3, the match will guaranteed fail
            println!("Updating robots.txt from {} to {}", self.current_domain, url_object.as_str());
            for _ in 0..3 {
                match self.robotstxt.fetch_new_robots_txt(url_object.as_str()) {
                    Ok(_) => {
                        self.current_domain = Self::convert_url_to_domain(&url_object);
                        break;
                    },
                    Err(_) => {
                        println!("Failed, retrying...");
                        continue
                    }
                }
            }
        }

        if !self.robotstxt.allows_url(&url) {
            return Err(RequestHandlerError::DisallowedByRobotsTxt)
        }

        let page_content: Vec<u8>;
        let new_url: String;
        
        match self.http_request.request(url_object.as_str()) {
            Ok(t) => {
                page_content = t.0;
                new_url = t.1;
            }
            Err(t) => return Err(RequestHandlerError::HTTPRequestError(t))
        }


        return Ok((page_content, new_url))

    }
}

impl<'a, 'b> SimpleRequestHandler<'a, 'b> {
    pub fn new(robotstxt: &'a mut dyn robots_txt::RobotsTXT, http_request_object: &'b http_request::HTTPRequest) -> Self {
        return SimpleRequestHandler {
            robotstxt: robotstxt,
            http_request: http_request_object,
            current_domain: String::from("")
        }
    }
    fn filter_url(url: &mut url::Url) {
        url.set_fragment(None);
        url.set_query(None);
        let _ = url.set_scheme("http");
    }

    fn convert_url_to_domain(url: &url::Url) -> String {
        let mut converted_url = url.clone();
        Self::filter_url(&mut converted_url);
        converted_url.set_path("");
        return converted_url.as_str().to_owned()
    }
}