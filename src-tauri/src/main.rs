// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;
use walkdir::WalkDir;
use serde::{Serialize, Deserialize};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've be greeted from Rust!", name)
}

#[tauri::command]
fn custom() {
    println!("Hello from custom command");
}

// mod extmod;

// use extmod::filenames::printdirectories;
// mod testwalkdir;
// use testwalkdir::create_file_cache;
// use testwalkdir::write_cache_to_file;



#[derive(Debug,Serialize,Deserialize)]
struct Filedict {
    is_file: bool,
    file_path: String,
}

impl Filedict {
    fn new(file_path: String) -> Self {
        let mut is_file = false;
        if file_path.contains(".") {
            is_file = true;
        }
        Filedict { is_file, file_path }
    }
}
#[tauri::command]
fn readcache()->String{
    let mut file_hashmap: HashMap<String,Vec<Filedict>>= HashMap::new();
    let mut drives= [ "D://","C://"];
    for j in drives{
        for i in WalkDir::new(j){
            match i {
                Ok(entry) => {
                    let path_entry = entry.path().to_path_buf();
                    let file_name = path_entry.file_name();
                    if let Some(file_name) = file_name {
                        let file_name = file_name.to_str().unwrap().to_string();
                        file_hashmap
                            .entry(file_name)
                            .or_insert(Vec::new())
                            .push(Filedict::new(path_entry.to_str().unwrap().to_string()));
                    } else {
                        continue;
                    }
                }
                Err(error) => {
                    println!("Error: {}", error)
                }
            }
        }
    }
    println!("{} entries", file_hashmap.len());
    let  json_result= serde_json::to_string(&file_hashmap).unwrap();
    return json_result;

}

fn main() {
    
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![custom, greet,readcache])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}
