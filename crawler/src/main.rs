mod robots_txt;
mod http_request;
mod request_handler;

fn main() {
    let mut httprequest: http_request::HTTPRequest = http_request::HTTPRequest::new();
    httprequest.set_user_agent("search.prushton.com/1.0 (https://github.com/prushton2/searchengine)");
    let robotstxt: &dyn robots_txt::RobotsTXT = &robots_txt::RobotsTXTCrate::new(httprequest.clone());
    let mut requesthandler: &mut dyn request_handler::RequestHandler = &mut request_handler::SimpleRequestHandler::new(robotstxt, &httprequest);

    loop {
        match requesthandler.fetch("https://example.com") {
            Ok(t) => println!("{}:\n{}", t.1, String::from_utf8_lossy(&t.0)),
            Err(t) => println!("Error fetching URL {:?}", t),
        }
    }
}