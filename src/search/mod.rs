pub mod content_searcher;
pub mod directory_matcher;
pub mod file_walker;
pub mod name_matcher;

use crate::cli::Cli;
use crate::utils::{SearchResult, SearchStats};
use anyhow::Result;
use rayon::prelude::*;
use regex::Regex;
use std::sync::atomic::{AtomicUsize, Ordering};

pub fn execute_search(cli: &Cli) -> Result<Vec<SearchResult>> {
    let stats = SearchStats {
        files_searched: AtomicUsize::new(0),
        matches_found: AtomicUsize::new(0),
    };

    let pattern = if cli.regex {
        Some(create_regex(&cli.query, cli.ignore_case)?)
    } else {
        None
    };

    crate::display::show_search_header(&cli.query, &cli.directory);

    let files = file_walker::collect_files(cli)?;
    let results = search_files(cli, &files, pattern.as_ref(), &stats);

    if !cli.no_stats {
        crate::display::show_stats(&stats);
    }

    Ok(results)
}

fn create_regex(pattern: &str, ignore_case: bool) -> Result<Regex> {
    let mut regex_builder = regex::RegexBuilder::new(pattern);
    regex_builder.case_insensitive(ignore_case);
    Ok(regex_builder.build()?)
}

fn search_files(
    cli: &Cli,
    files: &[walkdir::DirEntry],
    regex: Option<&Regex>,
    stats: &SearchStats,
) -> Vec<SearchResult> {
    files
        .par_iter()
        .filter_map(|entry| {
            let path = entry.path();
            stats.files_searched.fetch_add(1, Ordering::Relaxed);

            let mut result = SearchResult {
                file_path: path.to_path_buf(),
                matches_in_name: false,
                content_matches: Vec::new(),
                encoding_warning: None,
                is_directory: entry.file_type().is_dir(),
            };

            if entry.file_type().is_dir() {
                if cli.folders_only || cli.include_folders {
                    result.matches_in_name = directory_matcher::check_directory_match(
                        path,
                        &cli.query,
                        regex,
                        cli.ignore_case,
                    );
                }

                if result.matches_in_name {
                    stats.matches_found.fetch_add(1, Ordering::Relaxed);
                    return Some(result);
                } else {
                    return None;
                }
            }

            if !cli.content_only && !cli.folders_only {
                result.matches_in_name =
                    name_matcher::check_filename_match(path, &cli.query, regex, cli.ignore_case);
            }

            if !cli.name_only && !cli.folders_only && crate::utils::is_text_file(path) {
                if cli.utf8_only && !is_valid_utf8_file(path) {
                    return None;
                }

                match content_searcher::search_file_content(
                    path,
                    &cli.query,
                    regex,
                    cli.ignore_case,
                    cli.max_line_length,
                ) {
                    Ok(matches) => {
                        result.content_matches = matches;

                        if cli.show_encoding_warnings && !is_valid_utf8_file(path) {
                            result.encoding_warning =
                                Some("File contains non-UTF-8 characters".to_string());
                        }
                    }
                    Err(err) if err.to_string().contains("Line too long") => {
                        if cli.show_encoding_warnings {
                            result.encoding_warning = Some(format!("Skipped: {}", err));
                        }

                        return None;
                    }
                    Err(_) if cli.show_encoding_warnings => {
                        result.encoding_warning =
                            Some("Failed to read file (encoding issue)".to_string());
                    }
                    Err(_) => {}
                }
            }

            if result.matches_in_name || !result.content_matches.is_empty() {
                stats.matches_found.fetch_add(1, Ordering::Relaxed);
                Some(result)
            } else {
                None
            }
        })
        .collect()
}

fn is_valid_utf8_file(path: &std::path::Path) -> bool {
    if let Ok(_content) = std::fs::read_to_string(path) {
        true
    } else {
        false
    }
}
