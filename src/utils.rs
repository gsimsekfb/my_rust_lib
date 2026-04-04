
use rand::Rng;
use std::fs;
use std::path::Path;

/// pretty_print: _ is the separator
pub fn pp<T: std::fmt::Display>(num: T) -> String {
    num.to_string().as_bytes().rchunks(3).rev().map(std::str::from_utf8)
       .collect::<std::result::Result<Vec<&str>, _>>().unwrap().join("_")
}

pub fn print_type_of<T>(name: &str, _: &T) {
    println!("--- type of {}: {}", name, std::any::type_name::<T>())
}

pub fn random_tech_picture_file() -> String {
    let files = [
        get_files_all_sub_folders(user_dir() + "/My Drive/Tech - Pictures/Rust"),
        get_files_all_sub_folders(user_dir() + "/My Drive/Tech - Pictures/Concurrency"),
        get_files_all_sub_folders(user_dir() + "/My Drive/Tech - Pictures/Blockchain"),
        get_files_all_sub_folders(user_dir() + "/My Drive/Tech - Pictures/Async Programming - Tokio"),
        get_files_all_sub_folders(user_dir() + "/My Drive/Tech - Pictures/Back End"),
        get_files_all_sub_folders(user_dir() + "/My Drive/Tech - Pictures/Functional Programming"),
        get_files_all_sub_folders(user_dir() + "/My Drive/Tech - Pictures/Testing"),
    ].concat();
    let index = rand::rng().random_range(0..files.len());
    files[index].clone()
}

pub fn random_tech_picture_cpp() -> String {
    let files = [
        get_files_all_sub_folders(user_dir() + "/My Drive/Tech - Pictures/C++"),
        get_files_all_sub_folders(user_dir() + "/My Drive/Tech - Pictures/Performance"),
        get_files_all_sub_folders(user_dir() + "/My Drive/Tech - Pictures/Python"),
        get_files_all_sub_folders(user_dir() + "/My Drive/Tech - Pictures/Quantum"),
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

pub fn get_files_all_sub_folders(path: impl AsRef<Path>) -> Vec<String> {
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

fn user_dir() -> String { std::env::var("USERPROFILE").unwrap() }

pub fn open_jpg(file: &str) {
    std::process::Command::new("C:\\Program Files\\XnViewMP\\xnviewmp.exe")
        .arg(file)
        .spawn()
        .expect("Failed to open XnViewMP");
}