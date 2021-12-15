#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::path::PathBuf;
use acid_store::repo::{key::KeyRepo, OpenMode, OpenOptions};
use acid_store::store::{DirectoryConfig, DirectoryStore};
use std::io::{Read, Seek, SeekFrom, Write};
extern crate os_type;

struct Storage(KeyRepo<String>);


fn config_path() -> String {
    let mut result: String = String::new();
    let platform = whoami::platform();
    let username = whoami::username();
    match platform {
        whoami::Platform::MacOS => {
            result = format!("/Users/{}/.config/void_wallet/db", username)
        }
        whoami::Platform::Windows => {
            result = format!("C:/Users/{}/Documents/void_wallet/db", username)
        }
        whoami::Platform::Linux => {
            result = format!("/home/{}/.config/void_wallet/db", username)
        }
        _ => (),
    }
    result
}

#[tauri::command]
fn get_private_keys(storage: tauri::State<Storage>) -> Vec<u8> {
  let mut data = storage.0.object("private").unwrap();
  let mut _data = vec![];
  data.read_to_end(&mut _data).unwrap();
  return _data
}

#[tauri::command]
fn add_private_key(storage: tauri::State<Storage>, key: String) {
  let mut db = storage.0.insert(String::from("private"));
  db.write(key.as_bytes()).unwrap();
  db.commit().unwrap();
}

fn main() {
  let mut storage: KeyRepo<String> = OpenOptions::new()
    .mode(OpenMode::CreateNew)
    .open(&DirectoryConfig {path: PathBuf::from(config_path())})
    .unwrap();
  tauri::Builder::default()
    .manage(Storage(storage))
    .invoke_handler(tauri::generate_handler![get_private_keys, add_private_key])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
