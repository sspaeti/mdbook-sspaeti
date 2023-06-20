use std::error::Error;

pub fn check_link(url: &str) -> Result<String, Box<dyn Error>> {
    match reqwest::blocking::get(url) {
        Ok(response) => {
            if response.status().is_success() {
                Ok(format!("The URL is valid: {}", url))
            } else {
                Ok(format!("The URL returned an error: {}, {}", response.status(), response.url()))
            }
        },
        Err(err) => {
            if err.is_redirect() {
                Err(format!("Redirection error for URL {}: {}", url, err).into())
            } else if err.is_timeout() {
                Err(format!("Timeout error for URL {}: {}", url, err).into())
            } else if err.is_body() {
                Err(format!("Body error for URL {}: {}", url, err).into())
            } else if err.is_status() {
                Err(format!("Status code error for URL {}: {}", url, err).into())
            } else if err.is_builder() {
                Err(format!("Builder error for URL {}: {}", url, err).into())
            } else if err.is_decode() {
                Err(format!("Decode error for URL {}: {}", url, err).into())
            } else {
                Err(format!("Unknown error for URL {}: {}", url, err).into())
            }
        },
    }
}
