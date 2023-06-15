use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use mdbook::book::Book;

use pulldown_cmark::{Event, Options, Parser, Tag, CodeBlockKind};
use pulldown_cmark_to_cmark::cmark;

pub struct CombinedConverter {
    pub base_url: String,
}

impl CombinedConverter {
    pub fn new(base_url: String) -> CombinedConverter {
        CombinedConverter { base_url }
    }

    fn format_wikilink(&self, wikilink_text: &str) -> String {
        let link = wikilink_text.to_lowercase().replace(" ", "-");
        format!("[{}]({}{}/)", wikilink_text, self.base_url, link)
    }
}


impl Preprocessor for CombinedConverter {
    fn name(&self) -> &str {
        "combined-converter"
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
                let events = Parser::new_ext(&chapter.content, parser_options).collect::<Vec<_>>();
                let mut new_events = Vec::new();
                let mut in_wikilink = false;
                let mut in_admonition = false;
                let mut wikilink_text = String::new();
                let mut admonition_type = String::new();
                for event in events {
                    match event {
                        // Wikilink events
                        Event::Text(text) if text.starts_with("[[") && text.ends_with("]]") => {
                                    wikilink_text = text.trim_start_matches("[[").trim_end_matches("]]").to_string();
                                    let formatted_wikilink = self.format_wikilink(&wikilink_text);
                                    eprintln!("mdbook-sspaeti: Found wikilink '{}', replaced with '{}'", wikilink_text, formatted_wikilink);
                                    new_events.push(Event::Text(formatted_wikilink.into()));
                                    wikilink_text.clear();
                                },

                        // Admonition events
                        Event::Start(Tag::BlockQuote) => {
                            in_admonition = true;
                            new_events.push(Event::Text(format!("```admonish {}\n", admonition_type).into()));
                            new_events.push(Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(admonition_type.clone().into()))));
                        },
                        Event::End(Tag::BlockQuote) if in_admonition => {
                            in_admonition = false;
                            eprintln!("mdbook-sspaeti: Found admonition '{}'", admonition_type);
                            new_events.push(Event::End(Tag::CodeBlock(CodeBlockKind::Fenced(admonition_type.clone().into()))));
                            admonition_type.clear();
                        },
                        Event::Text(text) if in_admonition && text.starts_with("[!") => {
                            admonition_type = text.trim_start_matches("[!").trim_end_matches("]").to_string();
                        },
                        Event::Text(text) if in_admonition => {
                            new_events.push(Event::Text(text));
                        },

                        // Other events
                        _ => new_events.push(event),
                    }
                }
                let mut new_content = String::new();
                cmark(new_events.into_iter(), &mut new_content).unwrap();
                chapter.content = new_content;
            }
        });
        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer != "not-supported"
    }
}
