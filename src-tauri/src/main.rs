// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs::OpenOptions, io::Write};

use shared_model::FileWriterArgs;
use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    println!("Geetings: {}", name);
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn write_to_file(content: FileWriterArgs) -> Result<String, String> {
    if content.should_error {
        return Err("Requested to error".to_owned());
    }

    println!("Writing to file: {}", content.content);
    let mut options = OpenOptions::new();
    let mut file = options
        .create(true)
        .truncate(true)
        .write(true)
        .open("foo.txt")
        .map_err(|e| e.to_string())?;
    write!(&mut file, "{}", content.content).map_err(|e| e.to_string())?;
    Ok("Wrote to file!".to_owned())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, write_to_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
