// File scanner module
use walkdir::WalkDir;
use regex::Regex;
use serde::{Deserialize, Serialize};

const VIDEO_EXTENSIONS: &[&str] = &[
    "mp4", "mkv", "avi", "mov", "wmv", "flv", "webm", "m4v", "mpeg", "mpg",
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScannedFile {
    pub path: String,
    pub name: String,
    pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub files: Vec<ScannedFile>,
    pub total: usize,
}

/// Scan a directory for video files
pub fn scan_directory(path: &str) -> Result<ScanResult, String> {
    let path = std::path::Path::new(path);

    if !path.exists() {
        return Err("Directory does not exist".to_string());
    }

    if !path.is_dir() {
        return Err("Path is not a directory".to_string());
    }

    let mut files = Vec::new();

    for entry in WalkDir::new(path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let file_path = entry.path();

        if !file_path.is_file() {
            continue;
        }

        // Check extension
        let extension = file_path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase());

        let Some(ext) = extension else {
            continue;
        };

        if !VIDEO_EXTENSIONS.contains(&ext.as_str()) {
            continue;
        }

        // Get file info
        let metadata = match std::fs::metadata(file_path) {
            Ok(m) => m,
            Err(_) => continue,
        };

        let file_name = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        let path_str = file_path.to_string_lossy().to_string();

        files.push(ScannedFile {
            path: path_str,
            name: file_name,
            size: metadata.len(),
        });
    }

    let total = files.len();

    log::info!("Scanned {} video files from {}", total, path.display());

    Ok(ScanResult { files, total })
}

/// Extract title from filename
pub fn extract_title(filename: &str) -> String {
    // Remove extension
    let name = filename
        .rsplit_once('.')
        .map(|(name, _)| name)
        .unwrap_or(filename);

    // Replace common separators with spaces
    let re = Regex::new(r"[-_]").unwrap();
    let name = re.replace_all(name, " ");

    // Trim whitespace
    name.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_title() {
        assert_eq!(extract_title("my_video.mp4"), "my video");
        assert_eq!(extract_title("ABC-123.mkv"), "ABC 123");
    }
}