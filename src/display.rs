use crate::cli::Cli;
use crate::utils::{SearchResult, SearchStats};
use colored::*;
use std::path::Path;
use std::sync::atomic::Ordering;

pub fn show_search_header(query: &str, directory: &Path) {
    println!(
        "{}",
        format!(
            "🔍 Searching for '{}' in: {}",
            query.bright_cyan(),
            directory.display().to_string().bright_yellow()
        )
    );
}

pub fn show_results(results: &[SearchResult], cli: &Cli) {
    println!("\n{}", "=".repeat(80).bright_black());

    if results.is_empty() {
        println!("{}", "❌ No matches found".bright_red());
        return;
    }

    for result in results {
        show_file_result(result, &cli.query, cli.max_lines);
    }
}

pub fn show_stats(stats: &SearchStats) {
    let files_count = stats.files_searched.load(Ordering::Relaxed);
    let matches_count = stats.matches_found.load(Ordering::Relaxed);

    println!("\n{}", "=".repeat(80).bright_black());
    println!(
        "{} {} | {} {}",
        "📊 Files searched:".bright_blue(),
        files_count.to_string().bright_yellow(),
        "Matches found:".bright_blue(),
        matches_count.to_string().bright_green()
    );
}

fn show_file_result(result: &SearchResult, query: &str, max_lines: usize) {
    println!(
        "\n{} {}",
        "📁".bright_blue(),
        result.file_path.display().to_string().bright_white()
    );

    if let Some(ref warning) = result.encoding_warning {
        println!("   {} {}", "⚠️".bright_yellow(), warning.bright_yellow());
    }

    if result.matches_in_name {
        println!(
            "   {} {}",
            "✓".bright_green(),
            "Match in filename".bright_cyan()
        );
    }

    if !result.content_matches.is_empty() {
        let occurrences = if result.content_matches.len() == 1 {
            "occurrence".to_string()
        } else {
            "occurrences".to_string()
        };

        println!(
            "   {} {} ({} {})",
            "✓".bright_green(),
            "Matches in content".bright_cyan(),
            result.content_matches.len().to_string().bright_yellow(),
            occurrences.bright_yellow()
        );

        let display_count = std::cmp::min(max_lines, result.content_matches.len());
        for (line_num, line_content) in result.content_matches.iter().take(display_count) {
            let highlighted = highlight_matches(line_content, query);
            println!(
                "      {} {}: {}",
                "Line".bright_black(),
                line_num.to_string().bright_magenta(),
                highlighted
            );
        }

        if result.content_matches.len() > max_lines {
            let remaining = result.content_matches.len() - max_lines;
            println!(
                "      {} {}",
                "...".bright_black(),
                format!("and {} more", remaining).bright_black()
            );
        }
    }
}

fn highlight_matches(text: &str, query: &str) -> String {
    text.replace(query, &query.bright_red().to_string())
}
