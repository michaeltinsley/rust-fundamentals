use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file(input: &std::path::Path) {
    let file = File::open(input).unwrap_or_else(|error| {
        eprintln!(
            "Error: Failed to open file '{}': {}",
            input.display(),
            error
        );
        std::process::exit(1);
    });

    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                eprintln!("Error reading from file: {}", error);
                std::process::exit(1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn test_read_file() {
        let file_path = Path::new("test.txt");
        let content = "Hello, world!\nThis is a test file.";
        std::fs::write(file_path, content).unwrap();

        read_file(file_path);

        std::fs::remove_file(file_path).unwrap();
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
    // let file_path = args.file.to_str();

    // match file_path {
    // Some(path) => read_file(path),
    // None => panic!("Invalid file path"),
    // }
    read_file(&args.file);
}
