use device_query::{DeviceQuery, DeviceState, Keycode};
use std::thread;
use tauri::{Emitter, Manager};
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use std::time::Duration;

#[tauri::command]
async fn handle_close_requested(window: tauri::Window, is_show: bool) {
    println!("{:?}", window);
    if is_show {
        window.show().unwrap();
    } else {
        window.hide().unwrap();
    }
}

fn start_global_listener(app_handle: &tauri::AppHandle) {
    let app_handle = app_handle.clone();
    thread::spawn(move || {
        let device_state = DeviceState::new();
        let mut pressed_keys = Vec::new();
        loop {
            let keys: Vec<Keycode> = device_state.get_keys();
            if !keys.is_empty() {
                println!("keys: {:?}", keys);
            }
            if keys != pressed_keys {
                pressed_keys = keys.clone();
                let key_info = format!("{:?}", pressed_keys);
                let _ = app_handle.emit("global-key-event", key_info);
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, handle_close_requested])
        .setup(|app| {
            let app_handle = app.handle();
            start_global_listener(app_handle);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
