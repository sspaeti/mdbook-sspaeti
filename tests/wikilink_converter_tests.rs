mod dummy_book;

use crate::dummy_book::DummyBook;
use mdbook::MDBook;
use mdbook::preprocess::Preprocessor;
use mdbook_sspaeti::{WikilinkConverter};

#[test]
fn test_wikilink_converter_1() {
    let converter = WikilinkConverter::new("https://www.ssp.sh/brain".to_string());
    let dummy_book = DummyBook::new();
    let temp = dummy_book.build().unwrap();
    let mut md = MDBook::load(temp.path()).unwrap();
    md.with_preprocessor(converter);

    md.config.set("preprocessor.wikilink-converter.ssp-url", "https://www.ssp.sh/brain").unwrap();

    let got = md.build();

    assert!(got.is_ok());
}

#[test]
fn test_wikilink_converter_2() {
    let converter = WikilinkConverter::new("https://www.ssp.sh/brain".to_string());
    let dummy_book = DummyBook::new();
    let temp = dummy_book.build().unwrap();
    let mut md = MDBook::load(temp.path()).unwrap();
    md.with_preprocessor(converter);

    md.config.set("preprocessor.wikilink-converter.ssp-url", "https://www.ssp.sh/brain").unwrap();

    let got = md.build();

    assert!(got.is_ok());
    // You can add more assertions here...
}
