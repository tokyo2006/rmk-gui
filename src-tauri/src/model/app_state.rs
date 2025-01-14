use hidapi::HidApi;

pub struct AppState {
    pub hid_api: HidApi,
    // pub current_device: Option<>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            hid_api: HidApi::new().expect("Failed to create HID API")
        }
    }
}