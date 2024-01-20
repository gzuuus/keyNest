// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// FS
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::fs::OpenOptions;

// Nostr
use nostr_sdk::prelude::*;

// Encrypt
use magic_crypt::{new_magic_crypt, MagicCryptTrait};

// Bitcoin
extern crate bitcoin;
use std::str::FromStr;
use bitcoin::bip32::{ChildNumber, DerivationPath, Xpriv, Xpub};
use bitcoin::hex::FromHex;
use bitcoin::secp256k1::ffi::types::AlignedType;
use bitcoin::secp256k1::Secp256k1;

const ACCOUNT_PATH: &str = "./";

#[tauri::command]
fn read_file(name: &str) -> serde_json::Value {
  println!("Reading file: {}", name);
  let file_path = Path::new(ACCOUNT_PATH).join(name);
  let mut file = File::open(file_path).expect("Failed to open file");
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
                    Err(_) => {},
                }
            }
            Some(files)
        },
        Err(_) => None,
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

// Bip32 example https://github.com/rust-bitcoin/rust-bitcoin/blob/0.31.x/bitcoin/examples/bip32.rs

#[tauri::command]
fn derive_child_pub_from_xpub(xpub: &str, child_index: u32) -> String {
    let mut buf: Vec<AlignedType> = Vec::new();
    buf.resize(Secp256k1::preallocate_size(), AlignedType::zeroed());
    let secp = Secp256k1::preallocated_new(buf.as_mut_slice()).unwrap();

    let xpub: Xpub = xpub.parse().unwrap();

    let zero = ChildNumber::from_normal_idx(child_index).unwrap();
    let public_key = xpub.derive_pub(&secp, &[zero, zero]).unwrap().public_key;
    println!("Derived public key: {}", public_key);
    public_key.to_string()
}

#[tauri::command]
fn derive_child_xprv_from_xprv(xprv: &str, child_index: u32) -> String {
    let mut buf: Vec<AlignedType> = Vec::new();
    buf.resize(Secp256k1::preallocate_size(), AlignedType::zeroed());
    let secp = Secp256k1::preallocated_new(buf.as_mut_slice()).unwrap();
    
    let root = Xpriv::from_str(xprv).unwrap();
    let render_path = format!("m/44h/1237h/{}h/0/0", child_index);
    let path = DerivationPath::from_str(&render_path).unwrap();
    let child = root.derive_priv(&secp, &path).unwrap();
    println!("Derived public key: {}", child);
    child.to_string()
}

#[tauri::command]
fn calculate_xprv_from_seed(seed: &str) -> String {
    let network = bitcoin::Network::Bitcoin;
    let seed = Vec::from_hex(seed).unwrap();

    let mut buf: Vec<AlignedType> = Vec::new();
    buf.resize(Secp256k1::preallocate_size(), AlignedType::zeroed());

    let root = Xpriv::new_master(network, &seed).unwrap();

    let xprv = root.to_string();
    xprv
}

#[tauri::command]
fn calculate_xpub_from_seed(seed: &str)-> String {
   let network = bitcoin::Network::Bitcoin;
   let seed = Vec::from_hex(seed).unwrap();

   let mut buf: Vec<AlignedType> = Vec::new();
   buf.resize(Secp256k1::preallocate_size(), AlignedType::zeroed());
   let secp = Secp256k1::preallocated_new(buf.as_mut_slice()).unwrap();

   let root = Xpriv::new_master(network, &seed).unwrap();

   let path = DerivationPath::from_str("m/44h/1237h/0h/0/0").unwrap();
   let xpub = Xpub::from_priv(&secp, &root);
   println!("Public key at {}: {}", path, xpub);
   let xpub_string = xpub.to_string();
   xpub_string
}

#[tokio::main]
async fn main() -> Result<()> {
    // derive("1136d293ace78ab1037166963b1d4cb8db8322dff5dbd98d9dceab5676c10207");
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            read_file, 
            list_files, 
            write_json, 
            delete_file_by_name, 
            encrypt_string, 
            decrypt_cypher, 
            calculate_xprv_from_seed, 
            calculate_xpub_from_seed,
            derive_child_pub_from_xpub,
            derive_child_xprv_from_xprv
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}