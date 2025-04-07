use log::info;
use std::ffi::CString;

use crate::models::*;
use crate::utils::*;

#[tauri::command]
pub async fn get_vial_devices(state: tauri::State<'_, AppState>) -> CommandResult<Vec<VialDevice>> {
    let mut state = state.lock().await;
    match state.hid_api.refresh_devices() {
        Ok(_) => {
            let mut devices = Vec::new();
            for device_info in state.hid_api.device_list() {
                let serial_number = device_info.serial_number().unwrap_or("");
                if serial_number.contains(VIAL_SERIAL_NUMBER_MAGIC) && is_rawhid(&device_info) {
                    devices.push(VialDevice::new(
                        device_info.product_string().unwrap().to_string(),
                        device_info.path().to_owned(),
                    ));
                    info!(
                        "Found Vial device: {} with path: {:?}",
                        device_info.product_string().unwrap(),
                        device_info.path()
                    );
                }
            }
            Ok(devices)
        }
        Err(e) => Err(format!("Failed to refresh HID devices: {}", e)),
    }
}

#[tauri::command]
pub async fn connect_vial_device(state: tauri::State<'_, AppState>, path: CString) -> CommandResult<()> {
    let mut state = state.lock().await;

    // Open device
    let device = match state.hid_api.open_path(&path) {
        Ok(device) => device,
        Err(e) => return Err(format!("Failed to connect to selected device: {}", e)),
    };

    // Get layer count
    state.kbd_params.layers = match layer_count(&device) {
        Ok(layers) => layers,
        Err(e) => return Err(format!("Failed to get layer count: {}", e)),
    };

    // Get macro count
    state.kbd_params.macros = match macro_count(&device) {
        Ok(macros) => macros,
        Err(e) => return Err(format!("Failed to get macro count: {}", e)),
    };

    // Get payload
    state.kbd_params.payload = match get_vial_payload(&device) {
        Ok(payload) => payload,
        Err(e) => return Err(format!("Failed to get Vial info: {}", e)),
    };

    // Get rows
    state.kbd_params.rows = match state.kbd_params.payload["matrix"]["rows"].as_u64() {
        Some(rows) => rows as u8,
        None => return Err("Failed to get keyboard rows".to_string()),
    };

    // Get columns
    state.kbd_params.cols = match state.kbd_params.payload["matrix"]["cols"].as_u64() {
        Some(cols) => cols as u8,
        None => return Err("Failed to get keyboard columns".to_string()),
    };

    // Get key count
    state.kbd_params.keys = match key_count(&state.kbd_params.payload) {
        Ok(keys) => keys,
        Err(e) => return Err(format!("Failed to calculate key count: {}", e)),
    };

    state.current_device = Some(device);
    Ok(())
}

#[tauri::command]
pub fn get_gui_version(_state: tauri::State<'_, AppState>) -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[tauri::command]
pub async fn get_layer_count(state: tauri::State<'_, AppState>) -> CommandResult<u8> {
    let state = state.lock().await;
    if state.current_device.is_none() {
        return Err("Device not connected".to_string());
    }
    Ok(state.kbd_params.layers)
}

#[tauri::command]
pub async fn get_macro_count(state: tauri::State<'_, AppState>) -> CommandResult<u8> {
    let state = state.lock().await;
    if state.current_device.is_none() {
        return Err("Device not connected".to_string());
    }
    Ok(state.kbd_params.macros)
}

#[tauri::command]
pub async fn get_key_count(state: tauri::State<'_, AppState>) -> CommandResult<usize> {
    let state = state.lock().await;
    if state.current_device.is_none() {
        return Err("Device not connected".to_string());
    }
    Ok(state.kbd_params.keys)
}
