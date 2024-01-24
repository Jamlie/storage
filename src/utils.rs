use colored::Colorize;
use std::path::{Path, PathBuf};

pub fn is_file(path: &Path) -> bool {
    let metadata = match path.metadata() {
        Ok(metadata) => metadata,
        Err(_) => return false,
    };

    return metadata.is_file();
}

pub fn get_full_path(path: &Path) -> PathBuf {
    let name = match path.canonicalize() {
        Ok(full_path) => full_path,
        Err(e) => {
            eprintln!("Error: {}", e.to_string().red());
            return PathBuf::from("");
        }
    };

    return name;
}

pub fn file_size(path: &Path) -> u64 {
    let metadata = match path.metadata() {
        Ok(metadata) => metadata,
        Err(_) => return 0,
    };

    return metadata.len();
}
