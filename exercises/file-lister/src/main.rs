use clap::Parser;
use file_lister::list_dir;

#[derive(Parser)]
struct Args {
    #[arg(default_value = ".")]
    path: std::path::PathBuf,

    #[arg(short, long, default_value_t = false, help = "Output in JSON format")]
    json: bool,

    #[arg(
        short,
        long,
        default_value_t = false,
        help = "List directories recursively"
    )]
    recursive: bool,

    #[arg(short, long, default_value_t = false, help = "List hidden files")]
    all: bool,
}

fn main() {
    let args = Args::parse();
    list_dir(&args.path, args.json, args.recursive, args.all);
}
