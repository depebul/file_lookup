use anyhow::Result;
use regex::Regex;
use std::path::Path;

pub fn search_file_content(
    path: &Path,
    query: &str,
    regex: Option<&Regex>,
    ignore_case: bool,
) -> Result<Vec<(usize, String)>> {
    let content = match std::fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => {
            let bytes = std::fs::read(path)?;
            String::from_utf8_lossy(&bytes).to_string()
        }
    };

    let mut matches = Vec::new();

    for (line_num, line) in content.lines().enumerate() {
        let is_match = if let Some(regex) = regex {
            regex.is_match(line)
        } else if ignore_case {
            line.to_lowercase().contains(&query.to_lowercase())
        } else {
            line.contains(query)
        };

        if is_match {
            matches.push((line_num + 1, line.to_string()));
        }
    }

    Ok(matches)
}
