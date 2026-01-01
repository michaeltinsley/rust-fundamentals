use clap::Parser;
use file_size_formatter::Sizes;

#[derive(Parser)]
struct Cli {
    input: String,
}

fn main() {
    let args = Cli::parse();

    let sizes = Sizes::new(&args.input);
    println!("{:?}", sizes);
}
