// mod temp;

use rand::Rng;
use std::fs;
use std::path::Path;

pub fn random_file() -> String {
    let files = get_rust_files("C:/code/my_rust_lib/src/");
    let index = rand::rng().random_range(0..files.len());
    files[index].clone()
}

fn get_rust_files(path: impl AsRef<Path>) -> Vec<String> {
    let entries = fs::read_dir(path).unwrap();
    
    let rust_files: Vec<String> = entries
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "rs"))
        .filter_map(|entry| {
            entry.path()
                .file_name()
                .and_then(|name| name.to_str())
                .map(|s| s.to_string())
        })
        .collect();
    
    rust_files
}
