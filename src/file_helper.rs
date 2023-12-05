
use std::env::current_dir;
use std::fs;

pub fn get_file_contents (file_path: &str)-> Vec<String> {
    let root = current_dir().expect("failed to get current dir");
    let full_path = root.join(file_path);

    println!("Reading file {}", full_path.display());

    let contents = fs::read_to_string(full_path)
        .ok()
        .expect("Should have read the file, dog!");
    let lines: Vec<String> = contents.split("\n").map(|s: &str| s.to_string())
    .collect();
    return lines;
}