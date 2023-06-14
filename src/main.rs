use mdbook::book::Book;
use mdbook::errors::Error;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor, PreprocessorContext};
use regex::Regex;
use serde_json;
use std::io;
use std::process;
use clap;

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

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        eprintln!("Running {} preprocessor", self.name());
        let re = Regex::new(r"> \[!(\w+)\]\s*\n> (.*)").unwrap();
        book.for_each_mut(|item: &mut mdbook::BookItem| {
            if let mdbook::BookItem::Chapter(chapter) = item {
                let original_content = chapter.content.clone();
                chapter.content = re.replace_all(&chapter.content, "```admonish $1\n$2\n```").to_string();
                if original_content != chapter.content {
                    println!("Admonition converted in chapter '{}'", chapter.name);
                    println!("Original content: {}", original_content);
                    println!("Converted content: {}", chapter.content);
                }
            }
        });
        Ok(book)
    }


    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer != "not-supported"
    }
}

pub fn make_app() -> clap::App<'static> {
    clap::App::new("admonition-converter")
        .about("A mdbook preprocessor which converts Obsidian style admonitions to mdBook-admonish style")
        .subcommand(
            clap::SubCommand::with_name("supports")
                .arg(clap::Arg::with_name("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        )
}

fn main() {
    let matches = make_app().get_matches();

    let preprocessor = AdmonitionConverter::new();

    if let Some(sub_args) = matches.subcommand_matches("supports") {
        handle_supports(&preprocessor, sub_args);
    } else if let Err(e) = handle_preprocessing(&preprocessor) {
        eprintln!("{}", e);
        process::exit(1);
    }
}

fn handle_preprocessing(pre: &dyn Preprocessor) -> Result<(), Error> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;
    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;
    Ok(())
}

fn handle_supports(pre: &dyn Preprocessor, sub_args: &clap::ArgMatches) -> ! {
    let renderer = sub_args.value_of("renderer").expect("Required argument");
    let supported = pre.supports_renderer(renderer);

    if supported {
        process::exit(0);
    } else {
        process::exit(1);
    }
}