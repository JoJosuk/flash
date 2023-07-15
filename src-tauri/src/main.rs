// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::path::Path;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've be greeted from Rust!", name)
}

#[tauri::command]
fn custom() {
    println!("Hello from custom command");
}

fn printdirectories(){
    let current_dir= Path::new("D:/repos").to_path_buf();
    let files = fs::read_dir(current_dir).unwrap();
    for file in files{
        let file = file.expect("Failed to read file");
        let filename = file.file_name().to_owned().into_string().unwrap();
        println!("{}", filename);
    }
}

fn main() {
    
    printdirectories();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![custom, greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}
