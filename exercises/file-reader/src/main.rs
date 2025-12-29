use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file(input: &std::path::Path) -> std::io::Result<()> {
    let file = File::open(input)?;

    let reader = BufReader::new(file);
    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
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

        _ = read_file(file_path);

        std::fs::remove_file(file_path).unwrap();
    }
}

use clap::Parser;

/// Read a file and display its contents.
#[derive(Parser)]
struct Cli {
    /// The path to the file to read
    file: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    if let Err(e) = read_file(&args.file) {
        eprintln!(
            "Error: Failed to read file '{}': {}",
            args.file.display(),
            e
        );
        std::process::exit(1);
    }
}
