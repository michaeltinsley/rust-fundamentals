#[derive(Debug)]
#[allow(dead_code)]
pub struct Sizes {
    bytes: String,
    kilobytes: String,
    megabytes: String,
    gigabytes: String,
}

impl Sizes {
    /// Creates a new Sizes struct by parsing a string input like "24 mb"
    pub fn new(input: &str) -> Self {
        // 1. Split the input string into value and unit
        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts.len() != 2 {
            // In a real app, we might return a Result, but for this signature we panic
            panic!("Invalid input format. Expected: 'number unit' (e.g., '24 mb')");
        }

        // 2. Parse the numeric part
        let size_val: f64 = parts[0].parse().expect("Could not parse the number");
        let unit_str = parts[1].to_lowercase();

        // 3. Normalize everything to Bytes first
        let total_bytes: u64 = match unit_str.as_str() {
            "b" | "bytes" => size_val as u64,
            "kb" | "kilobytes" => (size_val * 1000.0) as u64,
            "mb" | "megabytes" => (size_val * 1_000_000.0) as u64,
            "gb" | "gigabytes" => (size_val * 1_000_000_000.0) as u64,
            _ => panic!("Unknown unit. Supported: bytes, kb, mb, gb"),
        };

        // 4. Generate the formatted struct
        Sizes::format_all(total_bytes)
    }

    /// Helper function to format the calculated bytes into all units
    fn format_all(total_bytes: u64) -> Self {
        Sizes {
            bytes: format!("{} bytes", total_bytes),
            kilobytes: format!("{:.0} kilobytes", total_bytes as f64 / 1000.0),
            megabytes: format!("{:.0} megabytes", total_bytes as f64 / 1_000_000.0),
            gigabytes: format!("{:.0} gigabytes", total_bytes as f64 / 1_000_000_000.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Import the parent struct and methods

    #[test]
    fn test_24_mb_logic() {
        // This matches the example output required in your lab
        let sizes = Sizes::new("24 mb");

        assert_eq!(sizes.bytes, "24000000 bytes");
        assert_eq!(sizes.kilobytes, "24000 kilobytes");
        assert_eq!(sizes.megabytes, "24 megabytes");
        // Note: 24MB is 0.024 GB, which formats to "0" with {:.0} precision
        assert_eq!(sizes.gigabytes, "0 gigabytes");
    }

    #[test]
    fn test_kilobytes_case_insensitive() {
        // "KB" should work just like "kb"
        let sizes = Sizes::new("300 KB");

        assert_eq!(sizes.bytes, "300000 bytes");
        assert_eq!(sizes.kilobytes, "300 kilobytes");
    }

    #[test]
    fn test_gigabytes_full_word() {
        // "gigabytes" should work
        let sizes = Sizes::new("2 gigabytes");

        assert_eq!(sizes.bytes, "2000000000 bytes");
        assert_eq!(sizes.gigabytes, "2 gigabytes");
    }

    #[test]
    #[should_panic(expected = "Invalid input format")]
    fn test_missing_unit_panics() {
        // Should panic because there is no unit
        Sizes::new("500");
    }

    #[test]
    #[should_panic(expected = "Could not parse the number")]
    fn test_invalid_number_panics() {
        // Should panic because "abc" is not a number
        Sizes::new("abc mb");
    }

    #[test]
    #[should_panic(expected = "Unknown unit")]
    fn test_unknown_unit_panics() {
        // Should panic because "petabytes" is not handled
        Sizes::new("10 petabytes");
    }
}
