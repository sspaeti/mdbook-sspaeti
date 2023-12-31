mod rest;
use mdbook::preprocess::{Preprocessor, PreprocessorContext, CmdPreprocessor};
use mdbook::book::{Book, BookItem};
use mdbook::errors::Error;
use serde::{Serialize, Deserialize};
use regex::Regex;
use std::io;
use toml;

use rest::check_link;


#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct WikilinkPreprocessorConfig {
    pub brain_base_url: String,
}

impl WikilinkPreprocessorConfig {
    pub fn from_preprocessor_context(ctx: &PreprocessorContext) -> Self {
        match &ctx.config.get_preprocessor("sspaeti") {
            Some(raw) => {
                match raw.get("brain-base-url") {
                    Some(url_value) => {
                        if let toml::Value::String(url) = url_value {
                            Self { brain_base_url: url.clone() }
                        } else {
                            Self::default()
                        }
                    },
                    None => Self::default(),
                }
            },
            None => Self::default(),
        }
    }
}

pub struct WikilinkPreprocessor;
// use std::process::Command;

impl Preprocessor for WikilinkPreprocessor {
    fn name(&self) -> &str {
        "wikilinks"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        eprintln!("mdbook-sspaeti: Running {} preprocessor", self.name());
        let config = WikilinkPreprocessorConfig::from_preprocessor_context(ctx);
        let brain_base_url = config.brain_base_url;
        eprintln!("mdbook-sspaeti: brain_base_url = {}", brain_base_url);

        let regex = Regex::new(r"\[\[(?P<note>[^\]]+)\]\]").unwrap();
        book.for_each_mut(|section: &mut BookItem| {
            if let BookItem::Chapter(ref mut ch) = *section {
                let replaced = regex.replace_all(&ch.content, |caps: &regex::Captures| {
                    let note = &caps["note"];
                    let link_md = format!("({}/{})", brain_base_url, note.to_lowercase().replace(" ", "-")); 

                    //check if the link exists
                    let link = format!("{}/{}", brain_base_url, note.to_lowercase().replace(" ", "-")); 
                    let link_clone = link.clone();
                    // eprintln!("mdbook-sspaeti: link = {}", &link_clone);
                    match check_link(&link_clone) {
                        Ok(message) => {
                            eprintln!("mdbook-sspaeti- check_link: {}", message);
                            // replace wikilink with markdown link
                            format!("[{}]{}", note, link_md)
                        }
                        Err(err) => {
                            eprintln!("mdbook-sspaeti - check_link ERROR: {}", err);
                            // return Err(Error::msg(format!("Link check failed: {}", err)));
                            String::from("")
                            // return Err(Error::msg(format!("mdbook-sspaeti - check_link ERROR: {}", err)));
                        }
                    }

                });
                ch.content = replaced.into_owned();
            }
        });
        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer != "not-supported-renderer"
    }
}

fn main() {
    let preprocessor = WikilinkPreprocessor;

    match CmdPreprocessor::parse_input(io::stdin()) {
        Ok((ctx, book)) => {
            if preprocessor.supports_renderer(&ctx.renderer) {
                match preprocessor.run(&ctx, book) {
                    Ok(processed_book) => {
                        serde_json::to_writer(io::stdout(), &processed_book).unwrap();
                    }
                    Err(e) => {
                        eprintln!("Error while running preprocessor: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error while parsing input: {}", e);
        }
    }
}
