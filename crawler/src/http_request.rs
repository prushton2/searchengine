// Handles making http requests. This is lower level than request_handler, with the intention of working around things like 3XX and content-language 

#[derive(Clone)]
pub struct HTTPRequest {
    user_agent: String
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum HTTPRequestError {
    FailedToFetchURL,
    FailedToHeadURL,
    FailedToRedirect(String),
    BadStatusCode(u16),
    MissingHeader(String),
    BadHeaderValue(String, String),
    CouldntConvertToBytes
}

impl HTTPRequest {
    pub fn new(ua: &str) -> Self {
        return HTTPRequest{
            user_agent: ua.to_string()
        }
    }
    
    pub fn get_user_agent(&self) -> &str {
        return &self.user_agent
    }

    pub fn request(&self, url: &str, depth: Option<i32>) -> Result<(Vec<u8>, String), HTTPRequestError> {
        let current_depth = match depth {
            Some(t) => t,
            None => 0
        };

        let client = reqwest::blocking::Client::builder()
            .user_agent(self.user_agent.clone())
            .build()
            .unwrap();
    
        let result = match client.head(url).send() {
            Ok(t) => t,
            Err(_) => return Err(HTTPRequestError::FailedToHeadURL)
        };

        if result.status().is_redirection() && current_depth < 5{
            let redirect_to = match result.headers().get("location") {
                Some(t) => match t.to_str() {
                    Ok(t) => t,
                    Err(t) => return Err(HTTPRequestError::FailedToRedirect(format!("Error getting redirect location: {}", t)))
                },
                None => return Err(HTTPRequestError::FailedToRedirect("Couldnt find location header".to_string()))
            };

            return self.request(redirect_to, Some(current_depth+1));
        }

        if result.status().is_client_error() || result.status().is_server_error() {
            return Err(HTTPRequestError::BadStatusCode(result.status().as_u16()));
        }

        let content_type = match result.headers().get("content-type") {
            Some(t) => t,
            None => return Err(HTTPRequestError::MissingHeader("content-type".to_string()))
        };

        if content_type.to_str().is_err() {
            return Err(HTTPRequestError::MissingHeader("content-type".to_string()))
        }

        if !content_type.to_str().unwrap().contains("text/html") && !content_type.to_str().unwrap().contains("text/plain"){
            return Err(HTTPRequestError::BadHeaderValue("content-type".to_string(), content_type.to_str().unwrap_or("[invalid UTF-8]").to_string()))
        }

        let content_lang = match result.headers().get("content-language") {
            Some(t) => t.to_str(),
            None => Ok("en")
        };

        match content_lang {
            Ok(t) => {
                if t != "en" {
                    return Err(HTTPRequestError::BadHeaderValue("content-language".to_string(), t.to_string()))
                }
            }
            Err(_) => {}
        };

        let content = match client.get(url).send() {
            Ok(t) => t,
            Err(_) => return Err(HTTPRequestError::FailedToHeadURL)
        };

        // loads entire page to memory - size limit?
        let bytes = match content.bytes() {
            Ok(t) => t,
            Err(_) => return Err(HTTPRequestError::CouldntConvertToBytes)
        };

        // returning the url lets us know what the actual url is when dereferencing 3XX Urls
        return Ok((bytes.to_vec(), url.to_owned()))
    }
}