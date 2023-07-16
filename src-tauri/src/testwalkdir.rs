use std::collections::HashMap;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub fn create_file_cache(root_dirs: &[&str]) -> HashMap<String, Vec<String>> {
    let mut cache: HashMap<String, Vec<String>> = HashMap::new();

    for &root_dir in root_dirs {
        for entry in WalkDir::new(root_dir).into_iter().filter_map(|entry| entry.ok()) {
            if entry.file_type().is_file() {
                let file_name = entry.file_name().to_string_lossy().to_string();
                let entry_path = entry.path().display().to_string();

                cache.entry(file_name).or_insert_with(Vec::new).push(entry_path);
            }
        }
    }

    cache
}

