use std::path::{Path, PathBuf};

/// Converts values for SQL INSERT
///
/// # Arguments
///
/// * 'values' - A vector of strings to be parsed.
///
/// # Returns
///
/// A new vector of strings with surrounding single quotes if it contains text.
pub fn convert_insert_values(values: Vec<String>) -> Vec<String> {
    let mut result = Vec::new();

    for item in values {
        if let Ok(parsed_int) = item.parse::<i32>() {
            result.push(parsed_int.to_string());
        } else if let Ok(parsed_float) = item.parse::<f64>() {
            result.push(parsed_float.to_string());
        } else if item.eq_ignore_ascii_case("true") {
            result.push("true".to_string());
        } else if item.eq_ignore_ascii_case("false") {
            result.push("false".to_string());
        } else {
            // if it's not true or false, surround it with single quotes and push it.
            result.push(format!("'{}'", item));
        }
    }

    result
}

/// Finds the target directory starting from the given directory.
///
/// This function recursively searches for the "target" directory starting from the given
/// directory (`start_dir`). It traverses the directory tree upwards until it finds the "target"
/// directory or reaches the root directory.
///
/// # Arguments
///
/// * `start_dir` - The starting directory to begin the search.
///
/// # Returns
///
/// * `Some(PathBuf)` - The path to the "target" directory if found.
/// * `None` - If the "target" directory is not found within the directory tree.
///
/// ```
/// use std::path::Path;
/// use njord::util::find_target_directory;
/// let start_dir = Path::new(".");
/// let target_dir = find_target_directory(start_dir);
/// ```
pub fn find_target_directory(start_dir: &Path) -> Option<PathBuf> {
    let mut current_dir = start_dir.to_path_buf();

    loop {
        let target_dir = current_dir.join("target");

        if target_dir.is_dir() {
            return Some(target_dir);
        }

        if !current_dir.pop() {
            return None;
        }
    }
}

