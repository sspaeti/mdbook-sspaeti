mod rest;
use mdbook_preprocessor::book::{Book, BookItem};
use mdbook_preprocessor::errors::Result;
use mdbook_preprocessor::{Preprocessor, PreprocessorContext};
use regex::Regex;
use std::io;

use rest::check_link;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct WikilinkPreprocessorConfig {
    pub brain_base_url: String,
    pub is_url_check: bool,
    pub images_base_path: String,
}

impl WikilinkPreprocessorConfig {
    pub fn from_preprocessor_context(ctx: &PreprocessorContext) -> Self {
        let mut config = Self::default();
        if let Ok(Some(url)) = ctx.config.get::<String>("preprocessor.sspaeti.brain-base-url") {
            config.brain_base_url = url;
        }
        if let Ok(Some(check)) = ctx.config.get::<bool>("preprocessor.sspaeti.is-url-check") {
            config.is_url_check = check;
        }
        if let Ok(Some(path)) = ctx.config.get::<String>("preprocessor.sspaeti.images-base-path") {
            config.images_base_path = path;
        } else {
            config.images_base_path = "/images".to_string();
        }
        config
    }
}

/// Escape HTML special characters in a string.
fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// Convert inline markdown links in caption text to HTML <a> tags.
/// Handles: [text](url) → <a href="url">text</a>
fn render_inline_markdown(text: &str) -> String {
    let link_re = Regex::new(r"\[(?P<text>[^\]]+)\]\((?P<url>[^\)]+)\)").unwrap();
    let bold_re = Regex::new(r"\*\*(?P<text>[^*]+)\*\*").unwrap();
    // After bold replacement, remaining *text* is italic
    let italic_re = Regex::new(r"\*(?P<text>[^*]+)\*").unwrap();
    let code_re = Regex::new(r"`(?P<text>[^`]+)`").unwrap();

    let result = link_re.replace_all(text, |caps: &regex::Captures| {
        format!(
            "<a href=\"{}\">{}</a>",
            &caps["url"],
            escape_html(&caps["text"])
        )
    });
    let result = bold_re.replace_all(&result, "<strong>${text}</strong>");
    let result = italic_re.replace_all(&result, "<em>${text}</em>");
    let result = code_re.replace_all(&result, "<code>${text}</code>");
    result.into_owned()
}

/// Resolve an image path: if it's already absolute or a URL, keep it;
/// otherwise prepend the configured images base path.
fn resolve_image_path(filename: &str, images_base_path: &str) -> String {
    if filename.starts_with('/')
        || filename.starts_with("http://")
        || filename.starts_with("https://")
    {
        filename.to_string()
    } else {
        format!("{}/{}", images_base_path, filename)
    }
}

/// Build a <figure> HTML block with optional width and caption.
fn build_figure(src: &str, alt: &str, caption: Option<&str>, width: Option<&str>) -> String {
    let style = match width {
        Some(w) => format!(" style=\"max-width: {}\"", escape_html(w)),
        None => String::new(),
    };
    let img = format!("<img src=\"{}\" alt=\"{}\"{}>", escape_html(src), escape_html(alt), style);
    match caption {
        Some(cap) => format!(
            "<figure>\n{}\n<figcaption>{}</figcaption>\n</figure>",
            img,
            render_inline_markdown(cap)
        ),
        None if width.is_some() => {
            // Width but no caption — still wrap in figure for consistent styling
            format!("<figure>\n{}\n</figure>", img)
        }
        None => img,
    }
}

/// Process image wiki-links: ![[image.webp]], ![[image.webp|caption]],
/// and ![[image.webp|caption|width=400px]]
fn process_image_wikilinks(content: &str, images_base_path: &str) -> String {
    let re = Regex::new(r"!\[\[(?P<inner>[^\]]+)\]\]").unwrap();
    re.replace_all(content, |caps: &regex::Captures| {
        let inner = &caps["inner"];
        let parts: Vec<&str> = inner.splitn(3, '|').collect();
        let filename = parts[0].trim();
        let caption = if parts.len() > 1 { Some(parts[1].trim()) } else { None };
        let width = if parts.len() > 2 {
            let attr = parts[2].trim();
            let width_re = Regex::new(r"width\s*=\s*(?P<val>\S+)").unwrap();
            width_re.captures(attr).map(|c| c["val"].to_string())
        } else {
            None
        };
        let path = resolve_image_path(filename, images_base_path);
        let alt = caption.unwrap_or(filename);
        build_figure(&path, alt, caption, width.as_deref())
    })
    .into_owned()
}

/// Process image attributes: ![alt](path){width=500px}
/// Bare filenames like `![alt](roapi.webp){width=500px}` are auto-resolved.
fn process_image_attributes(content: &str, images_base_path: &str) -> String {
    let re = Regex::new(
        r"!\[(?P<alt>[^\]]*)\]\((?P<path>[^\)]+)\)\{(?P<attrs>[^\}]+)\}"
    )
    .unwrap();
    re.replace_all(content, |caps: &regex::Captures| {
        let alt = &caps["alt"];
        let path = resolve_image_path(caps["path"].trim(), images_base_path);
        let attrs = &caps["attrs"];

        let width_re = Regex::new(r"width\s*=\s*(?P<val>[^\s,}]+)").unwrap();
        let width = width_re
            .captures(attrs)
            .map(|c| c["val"].to_string());

        build_figure(&path, alt, None, width.as_deref())
    })
    .into_owned()
}

/// Process image + caption pattern:
///   ![alt](path)
///   _caption text with [links](url) and **bold**_
///
/// The image and italic caption must be on adjacent lines (possibly with
/// a blank line in between). Converts to <figure> with <figcaption>.
/// Bare filenames like `![alt](roapi.webp)` are auto-resolved.
fn process_image_captions(content: &str, images_base_path: &str) -> String {
    // Match image line, optional blank line, then italic caption line
    // Also supports the {width=...} attribute on the image
    let re = Regex::new(
        r"(?m)^!\[(?P<alt>[^\]]*)\]\((?P<path>[^\)]+)\)(?:\{(?P<attrs>[^\}]+)\})?\s*\n(?:\s*\n)?_(?P<caption>[^_\n]+(?:\n[^_\n]+)*)_\s*$"
    )
    .unwrap();
    re.replace_all(content, |caps: &regex::Captures| {
        let alt = &caps["alt"];
        let path = resolve_image_path(caps["path"].trim(), images_base_path);
        let caption = &caps["caption"];

        let width = caps.name("attrs").and_then(|attrs| {
            let width_re = Regex::new(r"width\s*=\s*(?P<val>[^\s,}]+)").unwrap();
            width_re.captures(attrs.as_str()).map(|c| c["val"].to_string())
        });

        let effective_alt = if alt.is_empty() { caption } else { alt };
        build_figure(&path, effective_alt, Some(caption), width.as_deref())
    })
    .into_owned()
}

pub struct WikilinkPreprocessor;

impl Preprocessor for WikilinkPreprocessor {
    fn name(&self) -> &str {
        "wikilinks"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
        eprintln!("mdbook-sspaeti: Running {} preprocessor", self.name());
        let config = WikilinkPreprocessorConfig::from_preprocessor_context(ctx);
        let brain_base_url = config.brain_base_url;
        let is_url_check = config.is_url_check;
        let images_base_path = config.images_base_path;

        eprintln!(
            "mdbook-sspaeti: brain_base_url = {}, is_url_check = {}, images_base_path = {}",
            brain_base_url, is_url_check, images_base_path
        );

        // Regex for text wikilinks (not preceded by !)
        let wikilink_regex = Regex::new(r"\[\[(?P<note>[^\]]+)\]\]").unwrap();

        // Strip HTML comments from chapter content (author notes, TODOs, etc.)
        let html_comment_re = Regex::new(r"(?s)<!--.*?-->").unwrap();

        book.for_each_mut(|section: &mut BookItem| {
            if let BookItem::Chapter(ref mut ch) = *section {
                // 0. Strip HTML comments (but not inside fenced code blocks)
                //    Build a set of byte ranges that are inside fenced blocks
                let mut fenced_ranges: Vec<(usize, usize)> = Vec::new();
                let mut fence_start: Option<(usize, &str)> = None;
                let mut pos = 0;
                for line in ch.content.lines() {
                    let trimmed = line.trim_start();
                    if let Some((start, marker)) = fence_start {
                        if trimmed.starts_with(marker) && trimmed.trim() == marker {
                            // end of fenced block (pos + line.len() to include this line)
                            fenced_ranges.push((start, pos + line.len()));
                            fence_start = None;
                        }
                    } else if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
                        let marker_char = &trimmed[..1];
                        let marker_len = trimmed.chars().take_while(|&c| c.to_string() == marker_char).count();
                        let marker = &trimmed[..marker_len];
                        fence_start = Some((pos, marker));
                    }
                    pos += line.len() + 1; // +1 for newline
                }

                let content = &ch.content;
                let mut result = String::with_capacity(content.len());
                let mut last_end = 0;

                for comment in html_comment_re.find_iter(content) {
                    let in_fenced = fenced_ranges
                        .iter()
                        .any(|&(start, end)| comment.start() >= start && comment.end() <= end);
                    if in_fenced {
                        continue;
                    }
                    result.push_str(&content[last_end..comment.start()]);
                    last_end = comment.end();
                }
                result.push_str(&content[last_end..]);
                ch.content = result;

                // 1. Image wikilinks: ![[image.webp]] and ![[image.webp|caption]]
                //    Must run BEFORE text wikilinks to avoid partial matching
                ch.content = process_image_wikilinks(&ch.content, &images_base_path);

                // 2. Image + italic caption on next line → <figure>
                //    Must run BEFORE attribute processing (caption pattern includes optional attrs)
                ch.content = process_image_captions(&ch.content, &images_base_path);

                // 3. Image attributes: ![alt](path){width=500px}
                ch.content = process_image_attributes(&ch.content, &images_base_path);

                // 4. Text wikilinks: [[term]] and [[target|display]]
                let replaced =
                    wikilink_regex.replace_all(&ch.content, |caps: &regex::Captures| {
                        let note = &caps["note"];
                        let (link_target, display_name) = if let Some(pos) = note.find('|') {
                            (&note[..pos], &note[pos + 1..])
                        } else {
                            (note, note)
                        };
                        let link_md = format!(
                            "({}/{})",
                            brain_base_url,
                            link_target.to_lowercase().replace(" ", "-")
                        );

                        let link = format!(
                            "{}/{}",
                            brain_base_url,
                            link_target.to_lowercase().replace(" ", "-")
                        );
                        let link_clone = link.clone();
                        if is_url_check {
                            match check_link(&link_clone) {
                                Ok(message) => {
                                    eprintln!("mdbook-sspaeti- check_link: {}", message);
                                    format!("[{}]{}", display_name, link_md)
                                }
                                Err(err) => {
                                    eprintln!("mdbook-sspaeti - check_link ERROR: {}", err);
                                    String::from("")
                                }
                            }
                        } else {
                            format!("[{}]{}", display_name, link_md)
                        }
                    });
                ch.content = replaced.into_owned();
            }
        });
        Ok(book)
    }

    fn supports_renderer(&self, _renderer: &str) -> Result<bool> {
        Ok(true)
    }
}

fn main() {
    let preprocessor = WikilinkPreprocessor;

    match mdbook_preprocessor::parse_input(io::stdin()) {
        Ok((ctx, book)) => {
            match preprocessor.supports_renderer(&ctx.renderer) {
                Ok(true) => {
                    match preprocessor.run(&ctx, book) {
                        Ok(processed_book) => {
                            serde_json::to_writer(io::stdout(), &processed_book).unwrap();
                        }
                        Err(e) => {
                            eprintln!("Error while running preprocessor: {}", e);
                        }
                    }
                }
                _ => {
                    eprintln!("mdbook-sspaeti: renderer not supported");
                }
            }
        }
        Err(e) => {
            eprintln!("Error while parsing input: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_wikilink_simple() {
        let input = "![[roapi.webp]]";
        let result = process_image_wikilinks(input, "/images");
        assert_eq!(result, "<img src=\"/images/roapi.webp\" alt=\"roapi.webp\">");
    }

    #[test]
    fn test_image_wikilink_with_caption() {
        let input = "![[roapi.webp|ROAPI architecture overview]]";
        let result = process_image_wikilinks(input, "/images");
        assert!(result.contains("<figure>"));
        assert!(result.contains("<figcaption>ROAPI architecture overview</figcaption>"));
        assert!(result.contains("src=\"/images/roapi.webp\""));
    }

    #[test]
    fn test_image_wikilink_absolute_path() {
        let input = "![[/images/roapi.webp]]";
        let result = process_image_wikilinks(input, "/images");
        assert!(result.contains("src=\"/images/roapi.webp\""));
    }

    #[test]
    fn test_image_wikilink_url() {
        let input = "![[https://example.com/img.png|Example]]";
        let result = process_image_wikilinks(input, "/images");
        assert!(result.contains("src=\"https://example.com/img.png\""));
    }

    #[test]
    fn test_image_attributes_width() {
        let input = "![alt text](/images/foo.webp){width=500px}";
        let result = process_image_attributes(input, "/images");
        assert!(result.contains("style=\"max-width: 500px\""));
        assert!(result.contains("alt=\"alt text\""));
    }

    #[test]
    fn test_image_attributes_bare_filename() {
        let input = "![alt text](foo.webp){width=500px}";
        let result = process_image_attributes(input, "/images");
        assert!(result.contains("src=\"/images/foo.webp\""));
        assert!(result.contains("style=\"max-width: 500px\""));
    }

    #[test]
    fn test_image_attributes_percentage() {
        let input = "![alt](foo.webp){width=60%}";
        let result = process_image_attributes(input, "/images");
        assert!(result.contains("style=\"max-width: 60%\""));
    }

    #[test]
    fn test_image_caption_simple() {
        let input = "![alt](/images/foo.webp)\n_This is a caption_";
        let result = process_image_captions(input, "/images");
        assert!(result.contains("<figure>"));
        assert!(result.contains("<figcaption>This is a caption</figcaption>"));
    }

    #[test]
    fn test_image_caption_bare_filename() {
        let input = "![alt](foo.webp)\n_Caption for bare filename_";
        let result = process_image_captions(input, "/images");
        assert!(result.contains("src=\"/images/foo.webp\""));
        assert!(result.contains("<figcaption>Caption for bare filename</figcaption>"));
    }

    #[test]
    fn test_image_caption_with_link() {
        let input =
            "![alt](/images/foo.webp)\n_Image by [Staffbase](https://staffbase.com) about DWC_";
        let result = process_image_captions(input, "/images");
        assert!(result.contains("<figure>"));
        assert!(result.contains("<a href=\"https://staffbase.com\">Staffbase</a>"));
    }

    #[test]
    fn test_image_caption_with_width() {
        let input = "![alt](/images/foo.webp){width=400px}\n_Caption here_";
        let result = process_image_captions(input, "/images");
        assert!(result.contains("max-width: 400px"));
        assert!(result.contains("<figcaption>Caption here</figcaption>"));
    }

    #[test]
    fn test_image_caption_with_blank_line() {
        let input = "![alt](/images/foo.webp)\n\n_Caption after blank line_";
        let result = process_image_captions(input, "/images");
        assert!(result.contains("<figure>"));
        assert!(result.contains("<figcaption>Caption after blank line</figcaption>"));
    }

    #[test]
    fn test_image_wikilink_with_width() {
        let input = "![[roapi.webp|ROAPI overview|width=60%]]";
        let result = process_image_wikilinks(input, "/images");
        assert!(result.contains("<figure>"));
        assert!(result.contains("src=\"/images/roapi.webp\""));
        assert!(result.contains("max-width: 60%"));
        assert!(result.contains("<figcaption>ROAPI overview</figcaption>"));
    }

    #[test]
    fn test_render_inline_markdown() {
        assert_eq!(
            render_inline_markdown("by [Author](https://example.com) about **things**"),
            "by <a href=\"https://example.com\">Author</a> about <strong>things</strong>"
        );
    }

    #[test]
    fn test_strip_html_comment_single_line() {
        let html_comment_re = Regex::new(r"(?s)<!--.*?-->").unwrap();
        let input = "before <!-- TODO: fix this --> after";
        let result = html_comment_re.replace_all(input, "");
        assert_eq!(result, "before  after");
    }

    #[test]
    fn test_strip_html_comment_multiline() {
        let html_comment_re = Regex::new(r"(?s)<!--.*?-->").unwrap();
        let input = "before\n<!--\nTODO: first draft\nneeds rework\n-->\nafter";
        let result = html_comment_re.replace_all(input, "");
        assert_eq!(result, "before\n\nafter");
    }

    #[test]
    fn test_no_false_positive_wikilink() {
        // ![[image]] should NOT be caught by the text wikilink regex
        let wikilink_regex = Regex::new(r"\[\[(?P<note>[^\]]+)\]\]").unwrap();
        let input = "before ![[image.webp]] after";
        // After image wikilink processing, the [[...]] is gone
        let after_images = process_image_wikilinks(input, "/images");
        let matches: Vec<_> = wikilink_regex.find_iter(&after_images).collect();
        assert!(matches.is_empty());
    }
}
