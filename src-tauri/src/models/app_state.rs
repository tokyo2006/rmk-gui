use std::sync::Arc;

use hidapi::{HidApi, HidDevice};
use tauri::async_runtime::Mutex;

use super::KeyboardParams;

pub type AppState = Arc<Mutex<State>>;
pub struct State {
    pub hid_api:        HidApi,
    pub current_device: Option<HidDevice>,
    pub kbd_params:     KeyboardParams,
}

impl State {
    pub fn new() -> Self {
        Self {
            hid_api:        HidApi::new().expect("Failed to create HID API"),
            current_device: None,
            kbd_params:     KeyboardParams::new(),
        }
    }
}
