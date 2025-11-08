mod robots_txt;
mod http_request;
mod request_handler;
mod parser;

fn main() {
    crawler_thread();
}

fn crawler_thread() {
    let mut httprequest: http_request::HTTPRequest = http_request::HTTPRequest::new();
    httprequest.set_user_agent("search.prushton.com/1.0 (https://github.com/prushton2/searchengine)");
    let mut robotstxt: &mut dyn robots_txt::RobotsTXT = &mut robots_txt::RobotsTXTCrate::new(httprequest.clone());
    let mut requesthandler: &mut dyn request_handler::RequestHandler = &mut request_handler::SimpleRequestHandler::new(robotstxt, &httprequest);
    
    let mut limit = 0;

    while limit < 20 {
        limit += 1;

        let url = "https://example.com";

        let page_content: Vec<u8>;
        let new_url: String;
        match requesthandler.fetch(url) {
            Ok(t) => {
                page_content = t.0;
                new_url = t.1;
                println!("Fetched {}; Dereferenced to {}", url, new_url);
            }
            Err(t) => {
                println!("Error fetching URL {:?}", t);
                return;
            },
        }

        let parsed_content: parser::ParsedData = parser::parse_html(page_content, new_url);
    }
}