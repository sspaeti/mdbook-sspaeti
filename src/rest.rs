use std::error::Error;
use tokio::runtime::Runtime;
use url::Url;

pub fn check_link(url: &str) -> Result<String, Box<dyn Error>> {
    // Create a Tokio runtime
    let rt = Runtime::new()?;
    
    // Spawn a new Tokio task to execute the async code
    let result = rt.block_on(async move {
        // Parse the URL to handle relative URLs
        let parsed_url = Url::parse(url)?;
        // Make a GET request using reqwest

        // Make a GET request using reqwest
        let response = reqwest::get(parsed_url.as_str()).await?;
        
        // Check if the response was successful
        if response.status().is_success() {
            Ok(format!("The URL is valid: {}", url))
        } else {
            Ok(format!("The URL returned an error: {}", response.status()))
        }
    });

    match result {
        Ok(message) => {
            println!("{}", message);
            Ok(message)
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            Err(err)
        }
    }
}
