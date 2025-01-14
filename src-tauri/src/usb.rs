use hidapi::DeviceInfo;
use log::info;
use crate::model::*;

fn is_rawhid(device_info: &DeviceInfo) -> bool {
    device_info.usage_page() == VIAL_USAGE_PAGE_MAGIC && device_info.usage() == VIAL_USAGE_MAGIC
}

#[tauri::command]
pub fn get_vial_devices(state: tauri::State<'_, AppState>,) -> Vec<UsbDevice> {
    let mut devices = Vec::new();
    for device_info in state.hid_api.device_list() {
        let serial_number = device_info.serial_number().unwrap_or("");
        if serial_number.contains(VIAL_SERIAL_NUMBER_MAGIC) && is_rawhid(&device_info) {
            devices.push(UsbDevice::new(
                device_info.product_string().unwrap().to_string(),
                device_info.path().to_owned(),
                device_info.vendor_id(),
                device_info.product_id(),
            ));
            info!("Found Vial device: {} with path: {:?}", device_info.product_string().unwrap(), device_info.path());
        }
    }
    devices
}
