use std::fs;
use std::path::Path;
pub fn printdirectories(){
    let current_dir= Path::new("D:/repos").to_path_buf();
    let files = fs::read_dir(current_dir).unwrap();
    for file in files{
        let file = file.expect("Failed to read file");
        let filename = file.file_name().to_owned().into_string().unwrap();
        println!("{}", filename);
    }
}