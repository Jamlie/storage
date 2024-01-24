mod utils;

use colored::Colorize;
use std::env::args;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn size_in_mb(path: &Path) -> f64 {
    let mut size = 0.0;
    if utils::is_file(&path) {
        size = utils::file_size(&path) as f64;
        return size / 1024.0 / 1024.0;
    }

    for entry in WalkDir::new(&path) {
        let entry = entry.unwrap();
        let metadata = entry.metadata().unwrap();
        if metadata.is_file() {
            size += metadata.len() as f64;
        }
    }

    return size / 1024.0 / 1024.0;
}

fn execute(args: Vec<String>) {
    for (i, _) in args.iter().enumerate() {
        if i == 0 {
            continue;
        }
        let path_str = &args[i];
        let path = PathBuf::from(path_str);
        let full_path = utils::get_full_path(&path);
        if !path.exists() {
            eprintln!("{}", "Path does not exist".red());
            return;
        }

        let size = size_in_mb(&path);
        println!(
            "{}: {:.1} MB",
            if utils::is_file(&path) {
                full_path.to_str().unwrap().bright_yellow()
            } else {
                full_path.to_str().unwrap().bright_cyan()
            },
            size
        );
    }
}

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        eprintln!("{}", format!("Usage: {} <path> ...", args[0]).red());
        return;
    }

    execute(args);
}
