// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[tauri::command]
fn greet(name: &str) -> String {
  println!("Message from Rust: {}", name);
  format!("Hello, {}!", name)
}

#[tauri::command]
fn read_file(name: &str) -> serde_json::Value {
  println!("Reading file: {}", name);
  let mut file = File::open(name).expect("Failed to open file");
  let mut contents = Vec::new();
  file.read_to_end(&mut contents).expect("Failed to read file");

  serde_json::from_slice(&contents).expect("Failed to deserialize JSON")
}

#[tauri::command]
fn list_files() -> Vec<String> {
  let path = Path::new(".");
  let entries = fs::read_dir(path).expect("Failed to read directory");

  let mut files = Vec::new();
  for entry in entries {
    let entry = entry.expect("Failed to read directory entry");
    if entry.file_type().expect("Failed to get file type").is_file() {
      let file_name = entry.file_name().to_str().expect("Failed to convert file name to string").to_owned();
      if file_name.ends_with(".json") {
        files.push(file_name);
      }
    }
  }
  files
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet, read_file, list_files])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}