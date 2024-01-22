// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::InvokeError;
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

// DB
use rusqlite::{Connection, Result};

const ACCOUNT_PATH: &str = "./.nostr_accounts";

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
                            if file_name.ends_with(".db") {
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
    if !file_path.exists() {
        fs::create_dir_all(file_path.parent().unwrap()).unwrap();
    }

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path.to_str().unwrap())
        .unwrap();

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
fn decrypt_cypher(to_decrypt: &str, key: &str) -> Result<String, String> {
    let mc = new_magic_crypt!(key, 256);
    match mc.decrypt_base64_to_string(to_decrypt) {
        Ok(plain_text) => Ok(plain_text),
        Err(e) => Err(format!("Decryption failed: {}", e)),
    }
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

#[tauri::command]
fn insert_into_db(db_name: &str, name: &str, npub: &str, xpub: &str, prvk: &str, level: &str, gap: &str, parent: &str) -> bool {
    let render_db_name = format!("{}/{}{}", ACCOUNT_PATH, db_name, ".db");
    let conn = Connection::open(render_db_name).unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS identity (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            npub TEXT NOT NULL,
            xpub TEXT,
            prvk TEXT NOT NULL,
            level INTEGER,
            gap INTEGER,
            parent TEXT
        )",
        [],
    ).unwrap();

    match conn.execute(
        "INSERT INTO identity (name, npub, xpub, prvk, level, gap, parent) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7);",
        &[name, npub, xpub, prvk, level, gap, parent],
    ) {
        Ok(_) => true,
        Err(_) => false
    }
}

#[tauri::command]
fn account_count(db_name: &str) -> Result<usize, InvokeError> {
    let render_db_name = format!("{}/{}{}", ACCOUNT_PATH, db_name, ".db");
    let conn = Connection::open(render_db_name).unwrap();
    let count: usize = conn.query_row("SELECT count(*) FROM identity", [], |row| row.get(0)).unwrap();
    Ok(count)
}

struct Identity {
    id: i32,
    name: String,
    npub: String,
    xpub: String,
    prvk: String,
    level: i32,
    gap: i32,
    parent: String,
}

// TODO: fix this, make it flexible
#[tauri::command]
fn query_identity(db_name: &str,) {
    let render_db_name = format!("{}/{}{}", ACCOUNT_PATH, db_name, ".db");
    let conn = Connection::open(render_db_name).unwrap();
    let mut stmt = conn.prepare("SELECT * FROM identity WHERE name = 'hello';").unwrap();
    let identity_iter = stmt.query_map([], |row| {
        Ok(Identity {
            id: row.get(0)?,
            name: row.get(1)?,
            npub: row.get(2)?,
            xpub: row.get(3)?,
            prvk: row.get(4)?,
            level: row.get(5)?,
            gap: row.get(6)?,
            parent: row.get(7)?,
        })
    }).unwrap();
    for identity in identity_iter {
        let identity = identity.unwrap();
        println!("Found identity: id={}, name={}, npub={}, xpub={}, prvk={}, level={}, gap={}, parent={}", 
                 identity.id, identity.name, identity.npub, identity.xpub, identity.prvk, identity.level, identity.gap, identity.parent);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // open_db("db_try.db");
    // query_identity("hello");
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
            derive_child_xprv_from_xprv,
            insert_into_db,
            account_count,
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}