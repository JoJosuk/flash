// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]



// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've be greeted from Rust!", name)
}

#[tauri::command]
fn custom() {
    println!("Hello from custom command");
}

mod extmod;
use extmod::filenames::printdirectories;

fn main() {
    
    printdirectories();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![custom, greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}
