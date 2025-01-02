use rdev::{listen, Event, EventType, Key, Keyboard};
use std::thread;
use tauri::{Emitter, Manager};
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn start_global_listener(app_handle: &tauri::AppHandle) {
    let app_handle = app_handle.clone();
    thread::spawn(move || {
        if let Err(error) = listen(move |event| {
            if let EventType::KeyPress(key) = event.event_type {
                let key_str = format!("{:?}", key);
                app_handle.emit("global-key-event", key_str).unwrap();
            }
        }) {
            println!("Error: {:?}", error);
        }
    });
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            let app_handle = app.handle();
            start_global_listener(app_handle);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
