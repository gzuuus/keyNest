#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::InvokeError;
use serde::{Serialize, Deserialize};

// FS
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::fs::create_dir_all;
use std::sync::Mutex;
use num_cpus;
// Nostr
use nostr::nips::nip06::{FromMnemonic, GenerateMnemonic};
use nostr::nips::nip19::ToBech32;
use nostr::{FromBech32, Keys};
use nostr::SecretKey;
use nostr::nips::nip49;
// Bitcoin
extern crate bitcoin;
use std::str::FromStr;
use bitcoin::{bip32::{DerivationPath, Xpriv, Xpub}, hex::DisplayHex};
use bip39::Mnemonic;
use bitcoin::hex::FromHex;
use bitcoin::secp256k1::ffi::types::AlignedType;
use bitcoin::secp256k1::Secp256k1;

// DB
use rusqlite::{Connection, Result};

static ACCOUNT_PATH: Mutex<Option<String>> = Mutex::new(None);

// FS
#[tauri::command]
fn test_dir() -> String {
    println!("Testing directory, {}", ACCOUNT_PATH.lock().unwrap().as_ref().unwrap());
    let path = ACCOUNT_PATH.lock().unwrap().as_ref().unwrap().to_string();
    path
}
#[tauri::command]
fn read_file(db_name: &str) -> serde_json::Value {
  println!("Reading file: {}", db_name);
  let file_path = Path::new(ACCOUNT_PATH.lock().unwrap().as_ref().unwrap()).join(db_name);
  let mut file = File::open(file_path).expect("Failed to open file");
  let mut contents = Vec::new();
  file.read_to_end(&mut contents).expect("Failed to read file");

  serde_json::from_slice(&contents).expect("Failed to deserialize JSON")
}

#[tauri::command]
fn list_files() -> Option<Vec<String>> {
    match fs::read_dir(ACCOUNT_PATH.lock().unwrap().as_ref().unwrap()) {
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
 fn delete_file_by_name(filename: &str) -> bool {
     println!("Deleting file: {}", filename);
     let file_path = Path::new(ACCOUNT_PATH.lock().unwrap().as_ref().unwrap()).join(filename);
 
     match fs::remove_file(&file_path) {
         Ok(_) => true,
         Err(_) => false,
     }
 }

 // Encrypt and decrypt
 #[tauri::command]
 fn encrypt_string(to_encrypt: &str, key: &str) -> String {
    let seed = SecretKey::from_hex(to_encrypt).unwrap();
    let cypher_text = nip49::EncryptedSecretKey::new(&seed, key, 16, nip49::KeySecurity::Unknown).unwrap().to_bech32().unwrap();
    cypher_text
 }

 #[tauri::command]
 fn decrypt_cypher(to_decrypt: &str, key: &str) -> Result<String, String> {
    let encrypted_secret_key = nip49::EncryptedSecretKey::from_bech32(to_decrypt).unwrap();
    let secret_key = nip49::EncryptedSecretKey::to_secret_key(encrypted_secret_key, key);

    match secret_key {
        Ok(sk) => Ok(sk.to_string()),   // Return plain text as a String
        Err(err) => Err(format!("Failed to convert to SecretKey: {:?}", err))
    }
}

// Derivation
// Bip32 example https://github.com/rust-bitcoin/rust-bitcoin/blob/0.31.x/bitcoin/examples/bip32.rs

#[tauri::command]
fn derive_child_pub_from_xpub(xpub: &str, child_index: u32) -> String {
    let mut buf: Vec<AlignedType> = Vec::new();
    buf.resize(Secp256k1::preallocate_size(), AlignedType::zeroed());
    let secp = Secp256k1::preallocated_new(buf.as_mut_slice()).unwrap();

    let xpub: Xpub = xpub.parse().unwrap();

    let path = DerivationPath::from_str(&format!("m/44h/1237h/{}h/0/0", child_index)).unwrap();

    let public_key = xpub.derive_pub(&secp, &path).unwrap().public_key;
    // println!("Derived public key at: {}, {}", path, public_key);
    public_key.to_string()
}

#[tauri::command]
fn derive_child_seed_from_xprv(xprv: &str, child_index: u32) -> String {
    let mut buf: Vec<AlignedType> = Vec::new();
    buf.resize(Secp256k1::preallocate_size(), AlignedType::zeroed());
    let secp = Secp256k1::preallocated_new(buf.as_mut_slice()).unwrap();

    let xprv: Xpriv = Xpriv::from_str(xprv).unwrap();

    let path = DerivationPath::from_str(&format!("m/44h/1237h/{}h/0/0", child_index)).unwrap();

    let child_seed = xprv.derive_priv(&secp, &path).unwrap().private_key.display_secret();
    // println!("Derived priv key at: {}, {}",path, child_seed);
    child_seed.to_string()
}

#[tauri::command]
fn derive_child_xprv_from_xprv(xprv: &str, child_index: u32) -> String {
    let mut buf: Vec<AlignedType> = Vec::new();
    buf.resize(Secp256k1::preallocate_size(), AlignedType::zeroed());
    let secp = Secp256k1::preallocated_new(buf.as_mut_slice()).unwrap();
    
    let root = Xpriv::from_str(xprv).unwrap();

    let path = DerivationPath::from_str(&format!("m/44h/1237h/{}h/0/0", child_index)).unwrap();
    let child = root.derive_priv(&secp, &path).unwrap();
    // println!("Derived priv key at: {}, {}", path, child);
    child.to_string()
}

#[tauri::command]
fn calculate_xprv_from_seed(seed: &str) -> String {
    let mut buf: Vec<AlignedType> = Vec::new();
    buf.resize(Secp256k1::preallocate_size(), AlignedType::zeroed());
    let secp = Secp256k1::preallocated_new(buf.as_mut_slice()).unwrap();
    let network = bitcoin::Network::Bitcoin;
    let seed = Vec::from_hex(seed).unwrap();

    let mut buf: Vec<AlignedType> = Vec::new();
    buf.resize(Secp256k1::preallocate_size(), AlignedType::zeroed());

    let root = Xpriv::new_master(network, &seed).unwrap();

    let path = DerivationPath::from_str("m/44h/1237h/0h/0/0").unwrap();
    let xprv = root.derive_priv(&secp, &path).unwrap();
    // println!("Private key at {}: {}", path, xprv.to_string());
    xprv.to_string()
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
   let xprv = root.derive_priv(&secp, &path).unwrap();
   let xpub = Xpub::from_priv(&secp, &xprv);
//    println!("Public key at {}: {}", path, xpub);
   let xpub_string = xpub.to_string();
   xpub_string
}

#[tauri::command]
fn mnemonic_from_seed(seed: &str) -> String {
    let mnemonic = Mnemonic::from_entropy(&Vec::from_hex(seed).unwrap()).unwrap().to_string();
    // println!("Mnemonic: {}", mnemonic);
    mnemonic
}

#[tauri::command]
fn seed_from_mnemonic(mnemonic: &str, passphrase: Option<&str>) -> String {
    let norm_mnemonic = Mnemonic::parse(mnemonic).expect(&format!("Invalid mnemonic"));
    let word_count = norm_mnemonic.word_count();
    match word_count {
        12 => {
            let seed = Keys::from_mnemonic(mnemonic, Some(passphrase.unwrap_or(""))).unwrap().secret_key().unwrap().display_secret().to_string();
            // println!("Seed(12): {}", seed);
            seed
        }
        24 => {
            let seed = Mnemonic::to_entropy(&norm_mnemonic).to_lower_hex_string();
            // println!("Seed: {}", seed);
            seed
        }
        _ => {
            // println!("Invalid word count: {}", word_count);
            "".to_string()
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Id {
    mnemonic: String,
    seed: String,
}

#[tauri::command]
fn generate_id() -> Id {
    let mnemonic = Keys::generate_mnemonic(12).unwrap().to_string();
    let seed = Keys::from_mnemonic(mnemonic.as_str(), Some("")).unwrap().secret_key().unwrap().to_bech32().unwrap().to_string();
    Id {
        mnemonic,
        seed,
    }
}

// DB
#[tauri::command]
fn insert_into_db(db_name: &str, name: &str, hexpub: &str, xpub: Option<&str>, prvk: &str, level: &str, gap: Option<&str>, parent: Option<&str>, child_index: Option<&str>, comments: Option<&str>) -> bool {
    let render_db_name = format!("{}/{}", ACCOUNT_PATH.lock().unwrap().as_ref().unwrap(), db_name);
    let conn = Connection::open(render_db_name).unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS identity (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            hexpub TEXT NOT NULL,
            xpub TEXT NULL,
            prvk TEXT NOT NULL,
            level INTEGER,
            gap INTEGER NULL,
            parent TEXT NULL,
            child_index INTEGER NULL,
            comments TEXT NULL
        )",
        [],
    ).unwrap();

    match conn.execute(
        "INSERT INTO identity (name, hexpub, xpub, prvk, level, gap, parent, child_index, comments) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9);",
        &[name, hexpub, xpub.unwrap_or("NULL"), prvk, level, gap.unwrap_or("NULL"), parent.unwrap_or("NULL"), child_index.unwrap_or("NULL"), comments.unwrap_or("NULL")],
    ) {
        Ok(_) => true,
        Err(_) => false
    }
}

#[tauri::command]
fn account_count(db_name: &str) -> Result<usize, InvokeError> {
    let render_db_name = format!("{}/{}{}", ACCOUNT_PATH.lock().unwrap().as_ref().unwrap(), db_name, ".db");
    let conn = Connection::open(render_db_name).unwrap();
    let count: usize = conn.query_row("SELECT count(*) FROM identity", [], |row| row.get(0)).unwrap();
    Ok(count)
}

#[derive(Serialize, Deserialize)]
struct Identity {
    id: i32,
    name: String,
    hexpub: String,
    xpub: Option<String>,
    prvk: Option<String>,
    level: Option<i32>,
    gap: Option<i32>,
    parent: Option<String>,
    child_index: Option<i32>,
    comments: Option<String>,
}

#[tauri::command]
fn get_root_identity_from_db(db_name: &str) -> Result<Vec<Identity>, InvokeError> {
    let render_db_name = format!("{}/{}", ACCOUNT_PATH.lock().unwrap().as_ref().unwrap(), db_name);
    let conn = Connection::open(render_db_name).unwrap();
    let mut stmt = conn.prepare("SELECT * FROM identity WHERE level = '0';").unwrap();
    let identity = match stmt.query_row([], |row| {
        Ok(Identity {
            id: row.get(0)?,
            name: row.get(1)?,
            hexpub: row.get(2)?,
            xpub: row.get(3)?,
            prvk: row.get(4)?,
            level: row.get(5)?,
            gap: row.get(6)?,
            parent: row.get(7)?,
            child_index: None,
            comments: row.get(9)?,
        })
    }) {
        Ok(identity) => identity,
        Err(e) => return Err(InvokeError::from(format!("Error querying database: {}", e))),
    };
    Ok(vec![identity])
}

#[tauri::command]
fn get_identities_by_column_and_value(db_name: &str, column: &str, value: &str) -> Result<Vec<Identity>, InvokeError> {
    let render_db_name = format!("{}/{}{}", ACCOUNT_PATH.lock().unwrap().as_ref().unwrap(), db_name, ".db");
    let conn = Connection::open(render_db_name).unwrap();
    let render_query = format!("SELECT * FROM identity WHERE {} = '{}';", column, value);
    let mut stmt = conn.prepare(&render_query).unwrap();
    let identity_iter = match stmt.query_map([], |row| {
        Ok(Identity {
            id: row.get(0)?,
            name: row.get(1)?,
            hexpub: row.get(2)?,
            xpub: row.get(3)?,
            prvk: row.get(4)?,
            level: row.get(5)?,
            gap: row.get(6)?,
            parent: row.get(7)?,
            child_index: row.get(8)?,
            comments: row.get(9)?,
        })
    }) {
        Ok(identity_iter) => identity_iter,
        Err(e) => return Err(InvokeError::from(format!("Error querying database: {}", e))),
    };
    let mut identities = Vec::new();
    for identity in identity_iter {
        let identity = identity.unwrap();
        identities.push(identity);
    }
    Ok(identities)
}

#[tauri::command]
fn get_all_identities(db_name: &str) -> Result<Vec<Identity>, InvokeError> {
    let render_db_name = format!("{}/{}", ACCOUNT_PATH.lock().unwrap().as_ref().unwrap(), db_name);
    let conn = Connection::open(render_db_name).unwrap();
    let render_query = format!("SELECT * FROM identity WHERE level != '0' AND comments != 'DELETED';");
    let mut stmt = conn.prepare(&render_query).unwrap();
    let identity_iter = match stmt.query_map([], |row| {
        Ok(Identity {
            id: row.get(0)?,
            name: row.get(1)?,
            hexpub: row.get(2)?,
            xpub: row.get(3)?,
            prvk: row.get(4)?,
            level: row.get(5)?,
            gap: row.get(6)?,
            parent: row.get(7)?,
            child_index: row.get(8)?,
            comments: row.get(9)?,
        })
    }) {
        Ok(identity_iter) => identity_iter,
        Err(e) => return Err(InvokeError::from(format!("Error querying database: {}", e))),
    };
    let mut identities = Vec::new();
    for identity in identity_iter {
        let identity = identity.unwrap();
        identities.push(identity);
    }
    Ok(identities)
}

#[tauri::command]
fn update_identity_in_db(db_name: &str, column: &str, value: &str, new_value: &str, where_column: &str) -> bool {
    let render_db_name = format!("{}/{}", ACCOUNT_PATH.lock().unwrap().as_ref().unwrap(), db_name);
    let conn = Connection::open(render_db_name).unwrap();
    let render_query = format!("UPDATE identity SET {} = '{}' WHERE {} = '{}';", column, new_value, where_column, value);
    
    match conn.execute(&render_query, []) {
        Ok(_) => true,
        Err(_) => false
    }
}

// Nostr
// TODO
#[tauri::command]
fn mine_id(prefixes: Vec<String>, cores: usize) -> Vec<String> {
    let keys = Keys::vanity(prefixes, true, cores).unwrap();
    let public_key = keys.public_key().to_bech32().unwrap();
    // let seed = keys.secret_key().unwrap().display_secret().to_string();
    let secret_key = keys.secret_key().unwrap().to_bech32().unwrap();

    let result = vec![public_key, secret_key];
    println!("Public key: {:?}", result);
    result
}

#[tauri::command]
fn count_cpus() -> usize {
    let count = num_cpus::get();
    println!("Number of CPUs: {}", count);
    count
}


#[tokio::main]
async fn main() -> Result<()> {
    tauri::Builder::default()
        .setup(|app| {
            let path = app.path_resolver().app_data_dir().expect("Failed to get app data directory");
            let path = path.join(".nostr_accounts/");
            create_dir_all(&path).expect("Failed to create directory");
            println!("Path: {:#?}", path);
            let mut resolved_path = ACCOUNT_PATH.lock().unwrap();
            *resolved_path = Some(path.to_str().unwrap().to_string());

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            read_file, 
            list_files,
            delete_file_by_name, 
            encrypt_string, 
            decrypt_cypher, 
            calculate_xprv_from_seed, 
            calculate_xpub_from_seed,
            derive_child_pub_from_xpub,
            derive_child_xprv_from_xprv,
            derive_child_seed_from_xprv,
            mnemonic_from_seed,
            seed_from_mnemonic,
            insert_into_db,
            account_count,
            get_root_identity_from_db,
            get_identities_by_column_and_value,
            get_all_identities,
            update_identity_in_db,
            generate_id,
            test_dir,
            mine_id,
            count_cpus
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}