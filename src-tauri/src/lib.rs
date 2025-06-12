use std::fs::File;
use std::io::Write;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn create_file(path: &str) -> String {
    let mut file = File::create(path).unwrap();
    file.write_all(b"Hello, world!").unwrap();
    "File created".to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, create_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
