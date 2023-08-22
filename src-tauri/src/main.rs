// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs::OpenOptions, io::Write};

#[tauri::command]
fn greet(name: &str) -> String {
    println!("Geetings: {}", name);
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn write_to_file(content: &str) -> String {
    println!("Writing to file: {}", content);
    let mut options = OpenOptions::new();
    let mut file = options
        .create(true)
        .truncate(true)
        .write(true)
        .open("foo.txt")
        .unwrap();
    write!(&mut file, "{}", content).unwrap();
    "Wrote to file!".to_owned()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, write_to_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
