use regex::Regex;
use std::path::Path;

pub fn check_filename_match(
    path: &Path, 
    query: &str, 
    regex: Option<&Regex>, 
    ignore_case: bool
) -> bool {
    let filename = path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("");
    
    if let Some(regex) = regex {
        regex.is_match(filename)
    } else if ignore_case {
        filename.to_lowercase().contains(&query.to_lowercase())
    } else {
        filename.contains(query)
    }
}