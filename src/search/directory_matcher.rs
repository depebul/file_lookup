use regex::Regex;
use std::path::Path;

pub fn check_directory_match(
    path: &Path, 
    query: &str, 
    regex: Option<&Regex>, 
    ignore_case: bool
) -> bool {
    let dirname = path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("");
    
    if let Some(regex) = regex {
        regex.is_match(dirname)
    } else if ignore_case {
        dirname.to_lowercase().contains(&query.to_lowercase())
    } else {
        dirname.contains(query)
    }
}