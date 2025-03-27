use byteorder::{ByteOrder, LittleEndian};
use hidapi::{DeviceInfo, HidDevice};
use serde_json::Value;
use std::io::Read;
use xz2::read::XzDecoder;

use crate::models::*;

pub fn is_rawhid(device_info: &DeviceInfo) -> bool {
    device_info.usage_page() == VIAL_USAGE_PAGE_MAGIC && device_info.usage() == VIAL_USAGE_MAGIC
}

pub fn write_read(device: &HidDevice, data: &[u8]) -> Result<[u8; 32], String> {
    if data.len() > MSG_LEN {
        return Err("Operation not supported: data length exceeds limit".to_string());
    }

    let mut write_buffer = [0u8; MSG_LEN + 1];
    write_buffer[1..data.len() + 1].copy_from_slice(data);

    device
        .write(&write_buffer)
        .map_err(|e| format!("Device write failed: {}", e))?;

    let mut read_buffer = [0u8; MSG_LEN];

    device
        .read(&mut read_buffer)
        .map_err(|e| format!("Device read failed: {}", e))?;

    Ok(read_buffer)
}

pub fn layer_count(device: &HidDevice) -> Result<u8, String> {
    let result = write_read(device, &[VialCommand::GetLayerCount.into()])?;
    Ok(result[1])
}

pub fn macro_count(device: &HidDevice) -> Result<u8, String> {
    let result = write_read(device, &[VialCommand::GetMacroCount.into()])?;
    Ok(result[1])
}

pub fn key_count(payload: &Value) -> Result<usize, String> {
    let layout = &payload["layouts"]["keymap"];
    Ok(count_strings(layout))
}

pub fn get_vial_payload(device: &HidDevice) -> Result<Value, String> {
    // Get size
    let size = write_read(device, &[VialCommand::VialPrefix.into(), VialCommand::GetSize.into()])?;

    let size = LittleEndian::read_u32(&size) as usize;

    // Get payload
    let mut payload = Vec::with_capacity(size);
    for block in 0..size.div_ceil(MSG_LEN) {
        let data = write_read(
            device,
            &[
                VialCommand::VialPrefix.into(),
                VialCommand::GetDefinition.into(),
                (block as u8).into(),
            ],
        )?;

        payload.extend_from_slice(&data[0..MSG_LEN.min(size - block * MSG_LEN)]);
    }

    // Decompress
    let mut decompressor = XzDecoder::new(&payload[..]);
    let mut payload = String::new();

    decompressor
        .read_to_string(&mut payload)
        .map_err(|e| format!("Failed to decompress layout definition: {}", e))?;

    serde_json::from_str(&payload).map_err(|e| format!("Failed to parse keyboard layout JSON: {}", e))
}

pub fn count_strings(value: &Value) -> usize {
    match value {
        serde_json::Value::String(_) => 1,
        serde_json::Value::Array(arr) => arr.iter().map(count_strings).sum(),
        serde_json::Value::Object(obj) => obj.values().map(count_strings).sum(),
        _ => 0,
    }
}
