mod robots_txt;
mod http_request;
mod request_handler;
mod parser;

fn main() {
    let mut httprequest: http_request::HTTPRequest = http_request::HTTPRequest::new();
    httprequest.set_user_agent("search.prushton.com/1.0 (https://github.com/prushton2/searchengine)");
    let robotstxt: &dyn robots_txt::RobotsTXT = &robots_txt::RobotsTXTCrate::new(httprequest.clone());
    let mut requesthandler: &mut dyn request_handler::RequestHandler = &mut request_handler::SimpleRequestHandler::new(robotstxt, &httprequest);

    // loop {
    let page_content: Vec<u8>;
    let new_url: String;
    match requesthandler.fetch("https://example.com") {
        Ok(t) => {
            page_content = t.0;
            new_url = t.1;
        }
        Err(t) => {
            println!("Error fetching URL {:?}", t);
            return;
        },
    }

    let parsed_content: parser::ParsedData = parser::parse_html(page_content, new_url);
    


    // }
}