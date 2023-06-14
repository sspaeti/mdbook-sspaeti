mod sspaeti_converter;

use mdbook::errors::Error;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor};
use serde_json;
use std::io;
use std::process;
use clap;

use sspaeti_converter::CombinedConverter;

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

    let combined_converter = CombinedConverter::new("https://www.ssp.sh/brain".to_string());

    if let Some(sub_args) = matches.subcommand_matches("supports") {
        if check_supports(&combined_converter, sub_args) {
            process::exit(0);
        } else {
            process::exit(1);
        }
    } else {
        if let Err(e) = handle_preprocessing(&combined_converter) {
            eprintln!("Error during combined preprocessing: {}", e);
            process::exit(1);
        }
    }
}

fn check_supports(pre: &dyn Preprocessor, sub_args: &clap::ArgMatches) -> bool {
    let renderer = sub_args.value_of("renderer").expect("Required argument");
    pre.supports_renderer(renderer)
}
fn handle_preprocessing(pre: &dyn Preprocessor) -> Result<(), Error> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;
    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;
    Ok(())
}


