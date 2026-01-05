use std::fs;
use std::path::Path;

/// Simple directory lister with recursion and basic formatting.
/// ```rust
/// use file_lister::list_dir;
/// use std::path::Path;
///
/// let path = Path::new(".");
/// list_dir(path, false, false, false);
/// ```
pub fn list_dir(path: &Path, json: bool, recursive: bool, hidden: bool) {
    if !path.is_dir() {
        eprintln!("Error: '{}' is not a directory.", path.display());
        return;
    }
    // Start recursion with depth 0
    if let Err(e) = list_recursive(path, json, recursive, hidden, 0) {
        eprintln!("Error reading directory: {}", e);
    }
}

fn list_recursive(
    path: &Path,
    json: bool,
    recursive: bool,
    hidden: bool,
    depth: usize,
) -> std::io::Result<()> {
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let child_path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();
            let is_dir = child_path.is_dir();

            // Filter hidden files if not requested
            if !hidden && name.starts_with('.') {
                continue;
            }

            // Print formatted output directly
            if json {
                // Use the `json!` macro from `serde_json` for safe serialization.
                let output = serde_json::json!({
                    "name": &name,
                    "path": child_path.display().to_string(),
                    "is_dir": is_dir
                });
                println!("{}", output);
            } else {
                let indent = "  ".repeat(depth);
                let icon = if is_dir { "📁" } else { "📄" };
                println!("{}{} {}", indent, icon, name);
            }

            // Recurse if directory and recursive flag is set
            if recursive && is_dir {
                list_recursive(&child_path, json, recursive, hidden, depth + 1)?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_dir() {
        let path = Path::new(".");
        list_dir(path, false, false, false);
    }

    #[test]
    fn test_list_dir_recursive() {
        let path = Path::new(".");
        list_dir(path, false, true, false);
    }

    #[test]
    fn test_list_dir_hidden() {
        let path = Path::new(".");
        list_dir(path, false, false, true);
    }

    #[test]
    fn test_list_dir_json() {
        let path = Path::new(".");
        list_dir(path, true, false, false);
    }
}
