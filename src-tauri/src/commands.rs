use byteorder::BigEndian;
use byteorder::ByteOrder;
use log::info;
use serde_json::Value;
use std::ffi::CString;
use strum::IntoEnumIterator;

use crate::models::*;
use crate::utils::*;

#[tauri::command]
pub async fn get_vial_devices(state: tauri::State<'_, AppState>) -> Result<Vec<VialDevice>, ()> {
    let mut state = state.lock().await;
    state.hid_api.refresh_devices().unwrap();
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

#[tauri::command]
pub async fn connect_vial_device(state: tauri::State<'_, AppState>, path: CString) -> Result<(), ()> {
    let mut state = state.lock().await;
    let device = state.hid_api.open_path(&path).unwrap();
    state.kbd_params.layers = layer_count(&device);
    state.kbd_params.macros = macro_count(&device);
    state.kbd_params.payload = get_vial_payload(&device);
    state.kbd_params.rows = state.kbd_params.payload["matrix"]["rows"].as_u64().unwrap() as u8;
    state.kbd_params.cols = state.kbd_params.payload["matrix"]["cols"].as_u64().unwrap() as u8;
    state.kbd_params.keys = key_count(&state.kbd_params.payload);
    state.current_device = Some(device);

    Ok(())
}

#[tauri::command]
pub fn get_gui_version(_state: tauri::State<'_, AppState>) -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[tauri::command]
pub async fn get_layer_count(state: tauri::State<'_, AppState>) -> Result<u8, ()> {
    let state = state.lock().await;
    Ok(state.kbd_params.layers)
}

#[tauri::command]
pub async fn get_macro_count(state: tauri::State<'_, AppState>) -> Result<u8, ()> {
    let state = state.lock().await;
    Ok(state.kbd_params.macros)
}

#[tauri::command]
pub async fn get_key_count(state: tauri::State<'_, AppState>) -> Result<usize, ()> {
    let state = state.lock().await;
    Ok(state.kbd_params.keys)
}

#[tauri::command]
pub async fn get_layout(state: tauri::State<'_, AppState>) -> Result<Value, ()> {
    let state = state.lock().await;
    Ok(state.kbd_params.payload.clone())
}

#[tauri::command]
pub async fn update_keymap(state: tauri::State<'_, AppState>) -> Result<(), ()> {
    let mut state = state.lock().await;
    let device = state.current_device.as_ref().unwrap();
    let mut size = 2usize;
    size *= state.kbd_params.layers as usize;
    size *= state.kbd_params.rows as usize;
    size *= state.kbd_params.cols as usize;
    let mut keymap = Vec::with_capacity(size);
    for i in 0..size.div_ceil(BUFFER_FETCH_CHUNK) {
        let read_size = BUFFER_FETCH_CHUNK.min(size - i * BUFFER_FETCH_CHUNK);
        let mut msg = [0u8; 32];
        msg[0] = VialCommand::GetKeymapBuffer.into();
        BigEndian::write_u16(&mut msg[1..=2], (i * BUFFER_FETCH_CHUNK) as u16);
        msg[3] = read_size as u8;
        let data = write_read(device, &msg).unwrap();
        keymap.extend_from_slice(&data[4..4 + read_size]);
    }
    itertools::iproduct!(0..state.kbd_params.layers, 0..state.kbd_params.rows, 0..state.kbd_params.cols).for_each(
        |(layer, row, col)| {
            let offset = layer as usize * state.kbd_params.rows as usize * state.kbd_params.cols as usize * 2
                + row as usize * state.kbd_params.cols as usize * 2
                + col as usize * 2;
            let keycode = KeyCode::from(&keymap[offset..=offset + 1]);
            state.kbd_params.keymap_set.insert((layer, row, col), keycode);
        },
    );
    Ok(())
}

#[tauri::command]
pub async fn get_layout_keymap(state: tauri::State<'_, AppState>) -> Result<Vec<Key>, ()> {
    let state = state.lock().await;
    let kle: kle_serial::Keyboard =
        serde_json::from_value(state.kbd_params.payload.clone()["layouts"]["keymap"].clone()).unwrap();
    let mut keys = vec![];
    for kle_key in kle.keys {
        let position_x = (kle_key.x, kle_key.x2);
        let position_y = (kle_key.y, kle_key.y2);
        let width = (kle_key.width, kle_key.width2);
        let height = (kle_key.height, kle_key.height2);
        let rotation = kle_key.rotation;
        let layer = 0u8;
        let legends_text = kle_key.legends[0].as_ref().unwrap().text.clone();
        let mut parts = legends_text.split(',').map(|s| s.parse::<u8>().unwrap());
        let row = parts.next().unwrap();
        let col = parts.next().unwrap();
        let keycode = state.kbd_params.keymap_set.get(&(layer, row, col)).unwrap().to_owned();
        keys.push(Key {
            layer,
            row,
            col,
            position_x,
            position_y,
            width,
            height,
            rotation,
            keycode,
        });
    }
    Ok(keys)
}

#[tauri::command]
pub async fn get_keycode_list(_state: tauri::State<'_, AppState>) -> Result<Vec<(String, u16)>, ()> {
    let mut keycode_list = vec![];
    for keycode in KeyCode::iter() {
        keycode_list.push((keycode.to_string(), keycode as u16));
    }
    Ok(keycode_list)
}

#[tauri::command]
pub async fn set_keycode(state: tauri::State<'_, AppState>, layer: u8, row: u8, col: u8, keycode: u16) -> Result<(), ()> {
    let state = state.lock().await;
    let device = state.current_device.as_ref().unwrap();
    let mut msg = [0u8; 6];
    msg[0] = VialCommand::SetKeycode.into();
    msg[1] = layer;
    msg[2] = row;
    msg[3] = col;
    BigEndian::write_u16(&mut msg[4..=5], keycode);
    write_read(&device, &msg).unwrap();
    Ok(())
}
