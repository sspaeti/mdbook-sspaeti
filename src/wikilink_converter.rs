use mdbook::book::Book;
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use regex::Regex;#[derive(Debug, Clone)]

pub struct WikilinkConverter {
    base_url: String,
}

impl WikilinkConverter {
    pub fn new(base_url: String) -> WikilinkConverter {
        WikilinkConverter { base_url }
    }
}

impl Preprocessor for WikilinkConverter {
    fn name(&self) -> &str {
        "wikilink-converter"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        eprintln!("mdbook-sspaeti: Running {} preprocessor", self.name());
        let base_url = ctx.config.get("preprocessor")
                                  .and_then(|c| c.get("sspaeti"))
                                  .and_then(|s| s.get("base-url"))
                                  .and_then(|url| url.as_str())
                                  .unwrap_or(&self.base_url);  // Use the initial base_url if not found in config

        eprintln!("mdbook-sspaeti: base_url = {}", base_url);

        let re = Regex::new(r"\[\[(?P<note>[^\]]+)\]\]").unwrap();
        book.for_each_mut(|item: &mut mdbook::BookItem| {
            if let mdbook::BookItem::Chapter(chapter) = item {
                let new_content = re.replace_all(&chapter.content, |caps: &regex::Captures| {
                    let note = &caps["note"].to_lowercase().replace(" ", "-");
                    let result = format!("[{}]({}{}/)", &caps["note"], base_url, note);
                    eprintln!("mdbook-sspaeti: Replaced link, new content = {}", result);
                    result
                });
                chapter.content = new_content.into_owned();
            }
        });
        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer != "not-supported"
    }
}

