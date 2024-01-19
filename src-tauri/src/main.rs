// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use nostr_sdk::prelude::*;
use std::fs::OpenOptions;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};

const ACCOUNT_PATH: &str = "./";

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
                            if file_name.ends_with(".json") && !file_name.ends_with(".conf.json") {
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
 fn write_json(name: &str, data: serde_json::Value) -> bool {
     let file_path = Path::new(ACCOUNT_PATH).join(format!("{}.json", name));
     let file = OpenOptions::new().write(true).create(true).truncate(true).open(file_path.to_str().unwrap()).unwrap();
     serde_json::to_writer_pretty(&file, &data).unwrap();
     true
 }

 #[tauri::command]
 fn delete_file_by_name(filename: &str) -> bool {
     println!("Deleting file: {}", filename);
     let file_path = Path::new(ACCOUNT_PATH).join(filename);
 
     match fs::remove_file(&file_path) {
         Ok(_) => true,
         Err(_) => false,
     }
 }

 #[tauri::command]
 fn encrypt_string(to_encrypt: &str, key: &str) -> String {
    println!("Encrypting string: {}", to_encrypt);
    let mc = new_magic_crypt!(key, 256);
    let cypher_text =mc.encrypt_str_to_base64(to_encrypt);
    cypher_text
 }

 #[tauri::command]
 fn decrypt_cypher(to_decrypt: &str, key: &str) -> String {
    let mc = new_magic_crypt!(key, 256);
    let plain_text =mc.decrypt_base64_to_string(to_decrypt).unwrap();
    plain_text
 }
 
#[tokio::main]
async fn main() -> Result<()> {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![read_file, list_files, write_json, delete_file_by_name, encrypt_string, decrypt_cypher])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}