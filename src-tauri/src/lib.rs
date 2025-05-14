use std::fs::File;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(serde::Serialize)]
struct FileContent {
    content: String,
    size: usize
}

#[tauri::command]
fn read_file() -> Result<FileContent, String> {
    let file = std::fs::read_to_string("/home/kyle/test.txt");

    if let Err(e) = file {
        return Err(e.to_string());
    }

    let file = file.unwrap();

    Ok(FileContent { content: file.clone(), size: file.len() })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, read_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
