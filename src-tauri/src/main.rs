#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{fs, path::Path};

#[tauri::command]
fn generate() -> Result<String, String> {
    let result = app::generate();

    Ok(result)
}

#[tauri::command]
fn save(path: String, content: String) -> Result<&'static str, &'static str> {
    let file_path = Path::new(&path).join("pom.xml");

    let result = fs::write(file_path, &content);

    match result {
        Ok(()) => Ok("保存成功"),
        Err(_) => Err("保存失敗"),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate, save])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
