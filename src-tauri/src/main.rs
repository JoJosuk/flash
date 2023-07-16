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
mod testwalkdir;
use testwalkdir::create_file_cache;
use testwalkdir::write_cache_to_file;

fn main() {
    let root_dirs = [
        "C:/",  // Example root directories of the drives, modify accordingly
        "D:/",
        "E:/",
    ];

    let file_cache = create_file_cache(&root_dirs);

    if let Err(err) = write_cache_to_file(&file_cache) {
        eprintln!("Failed to write file cache: {}", err);
    } else {
        println!("File cache has been written successfully.");
    }
    printdirectories();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![custom, greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}
