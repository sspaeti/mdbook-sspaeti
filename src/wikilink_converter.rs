use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use mdbook::book::Book;

use pulldown_cmark::{Event, Options, Parser, Tag};
use pulldown_cmark_to_cmark::cmark;

pub struct WikilinkConverter {
    pub base_url: String,
}

impl WikilinkConverter {
    pub fn new(base_url: String) -> WikilinkConverter {
         WikilinkConverter { base_url }
    }

    fn format_wikilink(&self, wikilink_text: &str) -> String {
        let link = wikilink_text.to_lowercase().replace(" ", "-");
        format!("[{}]({}{}/)", wikilink_text, self.base_url, link)
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

            let parser_options = Options::all();
            book.for_each_mut(|item: &mut mdbook::BookItem| {
                if let mdbook::BookItem::Chapter(chapter) = item {
                    eprintln!("mdbook-sspaeti: Processing chapter {}", chapter.name);
                    let events = Parser::new_ext(&chapter.content, parser_options).collect::<Vec<_>>();
                    let mut new_events = Vec::new();
                    let mut in_wikilink = false;
                    let mut wikilink_text = String::new();
                    for event in events {
                        match event {
                            Event::Start(Tag::Link(_, _, _)) => {
                                eprintln!("mdbook-sspaeti: Found a start link tag");
                                in_wikilink = true;
                                wikilink_text.clear();
                            }
                            Event::End(Tag::Link(_, _, _)) => {
                                eprintln!("mdbook-sspaeti: Found an end link tag");
                                in_wikilink = false;
                                let formatted_wikilink = self.format_wikilink(&wikilink_text);
                                new_events.push(Event::Text(formatted_wikilink.into()));
                            }
                            Event::Text(text) if in_wikilink && text.starts_with("[[") && text.ends_with("]]") => {
                                eprintln!("mdbook-sspaeti: Found a wikilink text");
                                wikilink_text = text.trim_start_matches("[[").trim_end_matches("]]").to_string();
                            }
                            _ => new_events.push(event),
                        }
                    }
                    eprintln!("mdbook-sspaeti: Converting to new format");
                    let mut new_content = String::new();
                    cmark(new_events.into_iter(), &mut new_content).unwrap();
                    chapter.content = new_content;
                    eprintln!("mdbook-sspaeti: Conversion complete");
                }
            });
            eprintln!("mdbook-sspaeti: Finished preprocessing");
            Ok(book)
        }
    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer != "not-supported"
    }

}
