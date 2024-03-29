
use std::{fs::{self, File}, collections::HashMap, io::Read};
extern crate base64;
use crate::{Multimedia, formats};

pub fn list_selection(paths: Vec<String>) -> Vec<Multimedia> {
    let mut files: Vec<Multimedia> = vec![];
    let mut local_index: usize = 0;
    for entry in &paths {
        let path_buf: std::path::PathBuf = std::path::Path::new(entry).to_path_buf();
        if path_buf.is_file() {
            if let Some(multimedia) = get_multimedia_info(&path_buf, &formats::all_file_formats(), local_index) {
                files.push(multimedia);
                local_index += 1;
            }
        }
    }
    files
}

//            if let Some(multimedia) = get_multimedia_info(&entry, &formats::all_file_formats(), local_index) {


pub fn list_files(dir: &std::path::Path, extensions: Vec<&str>) -> Vec<Multimedia> {
    let mut files: Vec<Multimedia> = vec![];
    if let Ok(entries) = fs::read_dir(dir) {
        let mut local_index: usize = 0;
        for entry in entries {
            if let Ok(entry) = entry {
                let path: std::path::PathBuf = entry.path();
                if path.is_file() {
                    if let Some(multimedia) = get_multimedia_info(&path, &extensions, local_index) {
                        files.push(multimedia);
                        local_index += 1;
                    }
                }
            }
        }
    }
    return files;
}

fn get_multimedia_info(file: &std::path::PathBuf, types: &[&str], local_index: usize) -> Option<Multimedia> {
    let path: &&std::path::PathBuf = &file;
    let input: Option<&str> = file.to_str();
    let input_str: &str = match input {
        Some(s) => s,
        None => return None,
    };
    
    for format in types {
        if let Some(last_slash_index) = input_str.rfind('/') {
            let substring: &str = &input_str[last_slash_index + 1..];
            if substring.contains(format) {
                if let Ok(metadata) = fs::metadata(file) {
                    let content = encode_file(file.clone().into_os_string().into_string().unwrap());                   
                        let multimedia: Multimedia = Multimedia {
                        name:           substring.to_string(),
                        local_index:    local_index,
                        path: path.to_path_buf(),
                        description:    String::from("placeholder"),
                        author:         String::from("placeholder"),
                        format:         format.to_string(),
                        file_type:      String::from("type"),
                        dimensions:     (0, 0),
                        size_bytes:     metadata.len(),
                        metadata:       HashMap::new(),
                        content:        content,
                    };
                    return Some(multimedia);
                }
            }
        }
    }
    None
}

pub fn encode_file(path: String) -> String {
    let mut file: File = File::open(path).expect("Failed to open file");
    let mut file_data: Vec<u8> = Vec::new();
    file.read_to_end(&mut file_data).expect("Failed to read file");
    let encoded_file: String = base64::encode(&file_data);
    return encoded_file;
}

pub fn get_new_file_name(file_name: &String, format: &String) -> String {
    let mut parts = file_name.rsplitn(2, ".");
    let extension:  &str = parts.next().unwrap_or("");
    let base:       &str = parts.next().unwrap_or("");
    let mut copy_number: i32 = 0;
    let mut file_name_of_copy: String = base.to_owned();

    while std::path::Path::new(&get_full_name(&file_name_of_copy, extension)).exists() {
        copy_number += 1;
        file_name_of_copy = format!("{}_copy{}", file_name_of_copy, copy_number);
    }
    file_name_of_copy+= &(".".to_owned() + &extension);
    return file_name_of_copy;
}

fn get_full_name(file_name: &String, extension: &str) -> String {
    let full_name = file_name.to_owned() + &extension;
    return full_name;
}
