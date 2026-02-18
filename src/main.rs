use std::env;
use std::process;

fn main() {
    let mut args = env::args();
    let _exe = args.next();     // exxcutable filename

    let filename = match args.next() {
        Some(filename) => filename,
        None => {
            eprintln!("Usage: srt_to_text <filename>");
            process::exit(1);
        }
    };

    println!("input filename: {}", filename);
}
