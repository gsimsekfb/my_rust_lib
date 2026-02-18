// mod temp;

use rand::Rng;
use std::fs;
use std::path::Path;

pub fn random_tech_picture_file() -> String {
    let files = [
        get_files_all_sub_folders("C:/Users/gokha/My Drive/Tech - Pictures/Rust"),
        get_files_all_sub_folders("C:/Users/gokha/My Drive/Tech - Pictures/Blockchain"),
    ].concat();
    let index = rand::rng().random_range(0..files.len());
    files[index].clone()
}

pub fn random_my_rust_lib_file() -> String {
    let files = get_files("C:/code/my_rust_lib/src/");
    let index = rand::rng().random_range(0..files.len());
    files[index].clone()
}

pub fn get_files(path: impl AsRef<Path>) -> Vec<String> {
    let entries = fs::read_dir(path).unwrap();

    let files: Vec<String> = entries
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .filter_map(|entry| {
            entry.path().file_name().and_then(|name| name.to_str())
                .map(|s| s.to_string())
        })
        .collect();
    
    files
}

pub fn get_files_all_sub_folders(path: &str) -> Vec<String> {
    let mut files = Vec::new();

    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            files.push(path.to_string_lossy().to_string());
        } else if path.is_dir() {
            // One level deep
            for sub_entry in fs::read_dir(&path).unwrap() {
                let sub_entry = sub_entry.unwrap();
                let sub_path = sub_entry.path();
                if sub_path.is_file() {
                    files.push(sub_path.to_string_lossy().to_string());
                }
            }
        }
    }

    files
}

// todo: delete
pub fn get_rust_files(path: impl AsRef<Path>) -> Vec<String> {
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
