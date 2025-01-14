use tauri::generate_handler;

mod model;
mod usb;
mod utils;

use model::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state = AppState::new(); //也许需要使用Arc + Mutex?
    let builder = tauri::Builder::default().setup(|app| {
        if cfg!(debug_assertions) {
            app.handle().plugin(
                tauri_plugin_log::Builder::default()
                    .level(log::LevelFilter::Info)
                    .build(),
            )?;
        }
        Ok(())
    })
        .manage(state)
        .invoke_handler(generate_handler![usb::get_vial_devices]);
    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
