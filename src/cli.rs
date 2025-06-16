use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(name = "lookup")]
#[command(about = "File and content search tool for terminal CLI")]
pub struct Cli {
    /// Search term to look for
    #[arg(help = "Text to search for in file names and content")]
    pub query: String,

    /// Starting directory (defaults to current directory)
    #[arg(short, long, default_value = ".")]
    pub directory: PathBuf,

    /// Search only in file names
    #[arg(long)]
    pub name_only: bool,

    /// Search only in file content
    #[arg(long)]
    pub content_only: bool,

    /// Search only in folder/directory names
    #[arg(long)]
    pub folders_only: bool,

    /// Include folders in search results
    #[arg(long)]
    pub include_folders: bool,

    /// Use regular expressions
    #[arg(short, long)]
    pub regex: bool,

    /// Ignore case sensitivity
    #[arg(short, long)]
    pub ignore_case: bool,

    /// Maximum search depth
    #[arg(long)]
    pub max_depth: Option<usize>,

    /// File extensions to search (e.g. rs,txt,md)
    #[arg(long, value_delimiter = ',')]
    pub extensions: Option<Vec<String>>,

    /// Maximum number of lines to display per file
    #[arg(long, default_value = "5")]
    pub max_lines: usize,

    /// Don't show statistics
    #[arg(long)]
    pub no_stats: bool,

    /// Skip files that aren't valid UTF-8
    #[arg(long)]
    pub utf8_only: bool,

    /// Show encoding warnings for non-UTF-8 files
    #[arg(long)]
    pub show_encoding_warnings: bool,

    /// Skip files with lines longer than this limit (default: 10000)
    #[arg(long, default_value = "10000")]
    pub max_line_length: usize,
}
