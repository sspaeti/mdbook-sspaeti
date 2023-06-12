use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new("path_to_your_md_file.md");
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let output_path = Path::new("path_to_your_converted_md_file.md");
    let mut output_file = File::create(&output_path)?;

    for line in reader.lines() {
        let line = line?;
        if line.starts_with("> [!") {
            let admonition_type = line.trim_start_matches("> [!").trim_end_matches("]");
            writeln!(output_file, "```admonish {}", admonition_type)?;
        } else if line.starts_with("> ") {
            writeln!(output_file, "{}", line.trim_start_matches("> "))?;
        } else if line.trim().is_empty() {
            writeln!(output_file, "```")?;
        } else {
            writeln!(output_file, "{}", line)?;
        }
    }

    Ok(())
}

