use std::env::args;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn get_dir_size(path: &Path) -> f64 {
    let mut size = 0.0;
    for entry in WalkDir::new(&path) {
        let entry = entry.unwrap();
        let metadata = entry.metadata().unwrap();
        if metadata.is_file() {
            size += metadata.len() as f64;
        }
    }

    return size / 1024.0 / 1024.0;
}

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        println!("Usage: {} <path>", args[0]);
        return;
    }

    let path_str = &args[1];
    let path = PathBuf::from(path_str);
    let dir_name = match path.canonicalize() {
        Ok(full_path) => full_path,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    if !path.exists() {
        println!("Path does not exist");
        return;
    }

    let metadata = path.metadata().unwrap();
    if !metadata.is_dir() {
        let size = metadata.len() as f64;
        println!(
            "{}: {:.1} MB",
            dir_name.to_str().unwrap(),
            size / 1024.0 / 1024.0
        );
        return;
    }

    let size = get_dir_size(&path);
    println!("{}: {:.1} MB", dir_name.to_str().unwrap(), size);
}
