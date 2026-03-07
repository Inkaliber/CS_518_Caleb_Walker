use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::Path;

trait LineFormatter {
    fn format(&self, line_number: usize, line: &str) -> String;
}

struct NumberedLineFormatter;

impl LineFormatter for NumberedLineFormatter {
    fn format(&self, line_number: usize, line: &str) -> String {
        format!("{line_number}: {line}")
    }
}

fn clean_line(raw: &str) -> Option<&str> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}

fn make_output_name(input_name: &str) -> String {
    let path = Path::new(input_name);
    let parent = path.parent().unwrap_or(Path::new(""));
    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");

    parent
        .join(format!("{stem}_resulting_output.txt"))
        .to_string_lossy()
        .into_owned()
}

fn process_reader_to_writer<R, W, F>(
    reader: R,
    mut writer: W,
    formatter: &F,
) -> io::Result<()>
where
    R: BufRead,
    W: Write,
    F: LineFormatter,
{
    let mut line_number = 1;

    for line_result in reader.lines() {
        let raw = line_result?;
        if let Some(cleaned) = clean_line(&raw) {
            writeln!(writer, "{}", formatter.format(line_number, cleaned))?;
            line_number += 1;
        }
    }

    writer.flush()?;
    Ok(())
}

fn rewrite_file<F>(input_name: &str, formatter: &F) -> io::Result<String>
where
    F: LineFormatter,
{
    let input = File::open(input_name)?;
    let output_name = make_output_name(input_name);
    let output = File::create(&output_name)?;

    let reader = BufReader::new(input);
    let writer = BufWriter::new(output);

    process_reader_to_writer(reader, writer, formatter)?;
    Ok(output_name)
}

fn main() {
    let formatter = NumberedLineFormatter;

    match rewrite_file("input.txt", &formatter) {
        Ok(output_name) => println!("Wrote {}", output_name),
        Err(e) => eprintln!("Error: {}", e),
    }
}