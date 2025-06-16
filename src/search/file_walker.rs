use crate::cli::Cli;
use anyhow::Result;
use walkdir::{DirEntry, WalkDir};

pub fn collect_files(cli: &Cli) -> Result<Vec<DirEntry>> {
    let walker = WalkDir::new(&cli.directory)
        .max_depth(cli.max_depth.unwrap_or(usize::MAX))
        .into_iter()
        .filter_map(|e| e.ok());

    let entries: Vec<_> = walker
        .filter(|entry| {
            if cli.folders_only {
                entry.file_type().is_dir()
            } else if cli.include_folders {
                true
            } else {
                entry.file_type().is_file()
            }
        })
        .filter(|entry| {
            if entry.file_type().is_file() {
                filter_by_extension(entry, &cli.extensions)
            } else {
                true
            }
        })
        .collect();

    Ok(entries)
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
