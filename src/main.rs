use std::env;
use std::process;
use std::fs;

fn main() {
    let mut args = env::args();
    let _exe = args.next();     // Executable filename

    let filename = match args.next() {
        Some(filename) => filename,
        None => {
            eprintln!("Usage: srt_to_text <filename>");
            process::exit(1);
        }
    };

    println!("input filename: {}", filename);

    // Read all bytes until EOF
    let contents = match fs::read_to_string(&filename) {
        Ok(text) => text,
        Err(err) => {
            eprintln!("Failed to read {}: {}", filename, err);
            process::exit(1);
        }
    };

    // println!("contents: {contents}");
    let mut texts = Vec::new();

    for raw_line in contents.lines() {
        let line = raw_line.trim();

        let number_symbol: bool = line.chars().all(|c| c.is_ascii_digit());

        if !line.is_empty() && !line.contains("-->") && !number_symbol {
            texts.push(line.to_string());
        }
    }

    let result = texts.join(" ");

    println!("{result}");
}
