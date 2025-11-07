// Handles making http requests. This is lower level than request_handler, as this module works around 3XX 
// codes and UA

pub struct HTTPRequest {
    user_agent: String
}

impl HTTPRequest {
    pub fn set_user_agent(&mut self, ua: &str) {
        self.user_agent = ua.to_string();
    }

    pub fn get_user_agent(&self) -> &str {
        return &self.user_agent
    }

    pub fn request(&self, url: &str) -> Result<(Vec<u8>, String), String> {
        let client = reqwest::blocking::Client::builder()
            .user_agent(self.user_agent.clone())
            .build()
            .unwrap();
    
        let result = match client.get(url).send() {
            Ok(t) => t,
            Err(_) => return Err("Could not get URL".to_string())
        };

        if result.status().is_redirection() {
            let redirect_to = match result.headers().get("location") {
                Some(t) => match t.to_str() {
                    Ok(t) => t,
                    Err(t) => return Err(format!("Error getting redirect location: {}", t))
                },
                None => return Err("Couldnt find location header".to_string())
            };

            return self.request(redirect_to);
        }

        if result.status().is_client_error() || result.status().is_server_error() {
            return Err(format!("Status Code Invalid ({})", result.status().as_str()));
        }

        let content_type = match result.headers().get("content-type") {
            Some(t) => t,
            None => return Err("No content type header".to_string())
        };

        if content_type.to_str().is_err() {
            return Err("Error verifying content type header".to_string())
        }

        if !content_type.to_str().unwrap().contains("text/html") {
            return Err("Content type is not html".to_string())
        }

        let content_lang = match result.headers().get("content-language") {
            Some(t) => t.to_str(),
            None => Ok("en")
        };

        if content_lang.is_err() || content_lang.unwrap() != "en" {
            return Err("Content language is not english".to_string())
        }

        let bytes = match result.bytes() {
            Ok(t) => t,
            Err(_) => return Err("Could not get bytes".to_string())
        };

        // returning the url lets us know what the actual url is when dereferencing 3XX Urls
        return Ok((bytes.to_vec(), url.to_owned()))
    }
}