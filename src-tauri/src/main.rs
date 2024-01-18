// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use nostr_sdk::prelude::*;

const ACCOUNT_PATH: &str = "./accounts";

#[tauri::command]
fn read_file(name: &str) -> serde_json::Value {
  println!("Reading file: {}", name);
  let file_path = Path::new(ACCOUNT_PATH).join(name);
  let mut file = File::open(file_path).expect("Failed to open file");
    // let mut file = File::open(accountpath/name).expect("Failed to open file");
  let mut contents = Vec::new();
  file.read_to_end(&mut contents).expect("Failed to read file");

  serde_json::from_slice(&contents).expect("Failed to deserialize JSON")
}

#[tauri::command]
fn list_files() -> Option<Vec<String>> {
    match fs::read_dir(ACCOUNT_PATH) {
        Ok(entries) => {
            let mut files = Vec::new();
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        if entry.file_type().expect("Failed to get file type").is_file() {
                            let file_name = entry.file_name().to_str().expect("Failed to convert file name to string").to_owned();
                            if file_name.ends_with(".json") {
                                files.push(file_name);
                            }
                        }
                    },
                    Err(_) => {}, // handle error here if needed
                }
            }
            Some(files)
        },
        Err(_) => None, // return None if failed to read directory
    }
}

#[tauri::command]
fn write_json(name: &str, data: serde_json::Value) {
    let file_path = Path::new(ACCOUNT_PATH).join(format!("{}.json", name));
    let file = File::create(file_path).unwrap();
    serde_json::to_writer_pretty(&file, &data).unwrap();
}

#[tokio::main]
async fn main() -> Result<()> {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![read_file, list_files, write_json])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
  Ok(())
}