use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
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

pub fn write_cache_to_file(cache: &HashMap<String, Vec<String>>) -> io::Result<()> {
    let filecache_dir = Path::new("D:/filecache");
    fs::create_dir_all(&filecache_dir)?;

    let cache_file_path: PathBuf = filecache_dir.join("file_cache.txt");
    let mut cache_file = File::create(&cache_file_path)?;

    for (file_name, file_paths) in cache {
        writeln!(cache_file, "File: {}", file_name)?;
        for path in file_paths {
            writeln!(cache_file, "  - Path: {}", path)?;
        }
        writeln!(cache_file)?;
    }

    Ok(())
}

