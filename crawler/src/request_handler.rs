use crate::robots_txt;
use crate::http_request;

use url::Url;

pub trait RequestHandler<'a, 'b> {
    fn new(robotstxt: &'a dyn robots_txt::RobotsTXT, http_request_object: &'b http_request::HTTPRequest) -> Self where Self: Sized;
    fn fetch(&mut self, url: &str) -> Result<(Vec<u8>, String), String>;
}

pub enum RequestHandlerError {
    
}


struct SimpleRequestHandler<'a, 'b> {
    robotstxt: &'a dyn robots_txt::RobotsTXT,
    http_request: &'b http_request::HTTPRequest,
    current_Url: String
}

impl<'a, 'b> RequestHandler<'a, 'b> for SimpleRequestHandler<'a, 'b> {
    fn new(robotstxt: &'a dyn robots_txt::RobotsTXT, http_request_object: &'b http_request::HTTPRequest) -> Self {
        return SimpleRequestHandler {
            robotstxt: robotstxt,
            http_request: http_request_object,
            current_Url: String::from("")
        }
    }

    fn fetch(&mut self, url: &str) -> Result<(Vec<u8>, String), String> {
        let mut url_object = match Url::parse(url) {
            Ok(t) => t,
            Err(_) => return Err("".to_string())
        };
        Self::filter_url(&mut url_object);


        return Err(String::from(""))

    }
}

impl SimpleRequestHandler<'_, '_> {
    fn filter_url(url: &mut url::Url) {
        url.set_fragment(None);
        url.set_query(None);
        let _ = url.set_scheme("http");
    }

    fn convert_url_to_domain(url: &url::Url) -> url::Url {
        let mut converted_url = url.clone();
        Self::filter_url(&mut converted_url);
        converted_url.set_path("");
        return converted_url;
    }
}