#[derive(Debug)]
pub struct Sizes {
    bytes: u64,
    kilobytes: u64,
    megabytes: u64,
    gigabytes: u64,
    terabytes: u64,
}

impl Sizes {
    pub fn new(input: &str) -> Self {
        let components: Vec<&str> = input.split_whitespace().collect();

        let number = components[0].parse::<u64>().unwrap_or(0);
        let bytes = match components[1].to_lowercase().as_str() {
            "bytes" => number,
            "kb" => number * 1024,
            "mb" => number * 1024 * 1024,
            "gb" => number * 1024 * 1024 * 1024,
            "tb" => number * 1024 * 1024 * 1024 * 1024,
            _ => 0,
        };

        Sizes {
            bytes,
            kilobytes: bytes / 1024,
            megabytes: bytes / (1024 * 1024),
            gigabytes: bytes / (1024 * 1024 * 1024),
            terabytes: bytes / (1024 * 1024 * 1024 * 1024),
        }
    }
}
