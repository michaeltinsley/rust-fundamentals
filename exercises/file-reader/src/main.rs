use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file(input: &str) {
    let file = File::open(input);
    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                panic!("File not found: {}", error)
            }
            _ => {
                panic!("Error opening file: {}", error)
            }
        },
    };

    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                panic!("Error reading line: {}", error)
            }
        }
    }
}

use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The path to the file to read
    file: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let file_path = args.file.to_str();

    match file_path {
        Some(path) => read_file(path),
        None => panic!("Invalid file path"),
    }
}
