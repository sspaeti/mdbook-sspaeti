use mdbook::book::Book;
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use pulldown_cmark::{Event, Options, Parser, Tag, CodeBlockKind};
use pulldown_cmark_to_cmark::cmark;

pub struct AdmonitionConverter;

impl AdmonitionConverter {
    pub fn new() -> AdmonitionConverter {
        AdmonitionConverter
    }
}

impl Preprocessor for AdmonitionConverter {
    fn name(&self) -> &str {
        "admonition-converter"
    }

    // returning Markdown: preprocessor.admonish seems not to work with it
    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
            eprintln!("mdbook-sspaeti: Running {} preprocessor", self.name());
            let parser_options = Options::all();
            book.for_each_mut(|item: &mut mdbook::BookItem| {
                if let mdbook::BookItem::Chapter(chapter) = item {
                    let events = Parser::new_ext(&chapter.content, parser_options).collect::<Vec<_>>();
                    let mut new_events = Vec::new();
                    let mut in_admonition = false;
                    let mut admonition_type = String::new();
                    for event in events {
                        match event {
                            Event::Start(Tag::BlockQuote) => {
                                in_admonition = true;
                            }
                            Event::End(Tag::BlockQuote) => {
                                in_admonition = false;
                                new_events.push(Event::Text(format!("```admonish {}\n", admonition_type).into()));
                                new_events.push(Event::End(Tag::CodeBlock(CodeBlockKind::Fenced(admonition_type.clone().into()))));

                            }
                            Event::Text(text) if in_admonition && text.starts_with("[!") => {
                                admonition_type = text.trim_start_matches("[!").trim_end_matches("]").to_string();
                            }
                            Event::Text(text) if in_admonition => {
                                new_events.push(Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(admonition_type.clone().into()))));
                                new_events.push(Event::Text(text));
                            }
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

