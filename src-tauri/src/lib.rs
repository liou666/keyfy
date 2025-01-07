mod keycodes;
mod listen;

use tauri::{Emitter, Manager};

use listen::{listen, Event, EventType};

#[tauri::command]
async fn handle_close_requested(window: tauri::Window, is_show: bool) {
    if is_show {
        window.show().unwrap();
    } else {
        window.hide().unwrap();
    }
}

fn start_global_listener(app_handle: &tauri::AppHandle) {
    let app_handle = app_handle.clone();

    std::thread::spawn(move || {
        if let Err(error) = listen(
            move |event: Event| {
                if let Event {
                    event_type: EventType::KeyPress(key),
                    ..
                } = event
                {
                    let _ = app_handle.emit("global-key-event", format!(" {:?}", key));
                }
            },
            None,
        ) {
            eprintln!("Error: {:?}", error);
        }
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![handle_close_requested])
        .setup(|app| {
            let app_handle = app.handle();
            start_global_listener(&app_handle);

            let main_window = app_handle.get_webview_window("main").unwrap();
            let _ = main_window.set_skip_taskbar(true);
            let _ = main_window.set_shadow(false);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
