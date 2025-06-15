use crate::cli::Cli;
use anyhow::Result;
use walkdir::{DirEntry, WalkDir};

pub fn collect_files(cli: &Cli) -> Result<Vec<DirEntry>> {
    let walker = WalkDir::new(&cli.directory)
        .max_depth(cli.max_depth.unwrap_or(usize::MAX))
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|entry| entry.file_type().is_file());
    
    let files: Vec<_> = walker
        .filter(|entry| filter_by_extension(entry, &cli.extensions))
        .collect();
    
    Ok(files)
}

fn filter_by_extension(entry: &DirEntry, extensions: &Option<Vec<String>>) -> bool {
    if let Some(ref exts) = extensions {
        if let Some(ext) = entry.path().extension().and_then(|e| e.to_str()) {
            return exts.iter().any(|e| e.eq_ignore_ascii_case(ext));
        }
        return false;
    }
    true
}