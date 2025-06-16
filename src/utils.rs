use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicUsize;

#[derive(Debug)]
pub struct SearchResult {
    pub file_path: PathBuf,
    pub matches_in_name: bool,
    pub content_matches: Vec<(usize, String)>,
    pub encoding_warning: Option<String>,
}

pub struct SearchStats {
    pub files_searched: AtomicUsize,
    pub matches_found: AtomicUsize,
}

pub fn is_text_file(path: &Path) -> bool {
    if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
        let text_extensions = [
            "txt",
            "md",
            "rs",
            "py",
            "js",
            "ts",
            "html",
            "css",
            "json",
            "xml",
            "yml",
            "yaml",
            "toml",
            "cfg",
            "conf",
            "log",
            "csv",
            "sql",
            "sh",
            "bash",
            "zsh",
            "fish",
            "ps1",
            "bat",
            "cmd",
            "dockerfile",
            "makefile",
            "c",
            "cpp",
            "h",
            "hpp",
            "java",
            "go",
            "php",
            "rb",
            "pl",
            "lua",
            "r",
            "vim",
            "gitignore",
            "gitattributes",
            "editorconfig",
            "env",
            "ini",
        ];

        return text_extensions.contains(&extension.to_lowercase().as_str());
    }

    if let Some(file_name) = path.file_name().and_then(|name| name.to_str()) {
        let special_files = [
            "Makefile",
            "Dockerfile",
            "Cargo.lock",
            "package.json",
            "README",
            "LICENSE",
            "CHANGELOG",
            "AUTHORS",
            "CONTRIBUTORS",
            "Gemfile",
            "Rakefile",
            "CMakeLists.txt",
        ];

        if special_files.contains(&file_name) {
            return true;
        }
    }

    is_likely_text_file(path)
}

fn is_likely_text_file(path: &Path) -> bool {
    if let Ok(sample) = std::fs::read(path) {
        let sample_size = std::cmp::min(512, sample.len());
        if sample_size == 0 {
            return true;
        }

        let sample = &sample[..sample_size];

        let null_count = sample.iter().filter(|&&byte| byte == 0).count();
        if null_count > sample_size / 10 {
            return false;
        }

        if let Ok(_) = std::str::from_utf8(sample) {
            return true;
        }

        let printable_count = sample
            .iter()
            .filter(|&&byte| byte >= 32 && byte <= 126 || byte == 9 || byte == 10 || byte == 13)
            .count();

        printable_count > sample_size * 3 / 4
    } else {
        false
    }
}
