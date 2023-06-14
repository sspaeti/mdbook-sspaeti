use mdbook_sspaeti::{WikilinkConverter};
use std::fs;
use mdbook::book::{BookItem, Chapter};
use mdbook::config::Config;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use mdbook::book::{BookItem, Chapter};
    use mdbook::config::Config;

    #[test]
    fn test_wikilink_converter_1() {
        let converter = WikilinkConverter::new("https://www.ssp.sh/brain".to_string());

        let mut book = Book::new();
        // Read the file's contents
        let chapter_content = fs::read_to_string("~/Documents/git/book/DEDP/src/part_1/intro-convergent-evolution.md")
            .expect("Failed to read markdown file");

        // Create a Chapter instance with the file's contents
        let chapter = Chapter {
            name: String::from("Test Chapter"),
            content: chapter_content,
            number: None,
            sub_items: Vec::new(),
            path: None,
            source_path: None,
            nav_path: None,
            parent_names: Vec::new(),
        };

        // Add the chapter to the book
        book.push_item(BookItem::Chapter(chapter));

        // Create a dummy context
        let ctx = PreprocessorContext::new(&Config::default());

        let result = converter.run(&ctx, book);

        assert!(result.is_ok());
        // Add more assertions here to check that the book content has been correctly modified...
    }

    #[test]
    fn test_wikilink_converter_2() {
        let converter = WikilinkConverter::new("https://www.ssp.sh/brain".to_string());

        let mut book = Book::new();
        let chapter_content = String::from("[[note]]");

        let chapter = Chapter::new(
            String::from("Test Chapter"),
            chapter_content,
            PathBuf::from(""),
        );

        book.push_item(BookItem::Chapter(chapter));

        let ctx = PreprocessorContext::new(PathBuf::from(""), Config::default(), "html".to_string());

        let result = converter.run(&ctx, book);

        assert!(result.is_ok());
        let book = result.unwrap();
        if let Some(BookItem::Chapter(chapter)) = book.iter().next() {
            assert_eq!(chapter.content, "[note](https://www.ssp.sh/brain/note/)");
        } else {
            panic!("Expected a chapter in the book");
        }
    }
}
