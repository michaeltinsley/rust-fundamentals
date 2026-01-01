#[derive(Debug)]
pub struct Sizes {
    bytes: String,
    kilobytes: String,
    megabytes: String,
    gigabytes: String,
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
            _ => 0,
        };

        Sizes {
            bytes: format!("{} bytes", bytes),
            kilobytes: format!("{} kilobytes", bytes / 1024),
            megabytes: format!("{} megabytes", bytes / (1024 * 1024)),
            gigabytes: format!("{} gigabytes", bytes / (1024 * 1024 * 1024)),
        }
    }
}
