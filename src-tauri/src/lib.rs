use std::sync::Arc;

use tauri::{AppHandle, Manager, async_runtime::Mutex, generate_handler};

mod commands;
mod models;
mod utils;

use models::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state: AppState = Arc::new(Mutex::new(State::new()));
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = show_window(app);
        }))
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle()
                    .plugin(tauri_plugin_log::Builder::default().level(log::LevelFilter::Info).build())?;
            }
            Ok(())
        })
        .manage(state)
        .invoke_handler(generate_handler![
            commands::connect_vial_device,
            commands::get_gui_version,
            commands::get_vial_devices,
            commands::get_layer_count,
            commands::get_macro_count,
            commands::get_layout,
            commands::get_key_count,
            commands::update_keymap,
            commands::get_layout_keymap,
            commands::get_keycode_list,
            commands::set_keycode,
        ]);
    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn show_window(app: &AppHandle) {
    app.webview_windows()
        .values()
        .next()
        .expect("Sorry, no window found")
        .set_focus()
        .expect("Can't Bring Window to Focus");
}
