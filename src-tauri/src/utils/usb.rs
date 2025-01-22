use std::io::Read;
use byteorder::{ByteOrder, LittleEndian};
use hidapi::{DeviceInfo, HidDevice};
use log::info;
use serde_json::Value;
use xz2::read::XzDecoder;

use crate::models::*;

pub fn is_rawhid(device_info: &DeviceInfo) -> bool {
    device_info.usage_page() == VIAL_USAGE_PAGE_MAGIC && device_info.usage() == VIAL_USAGE_MAGIC
}

pub fn write_read(device: &HidDevice, data: &[u8]) -> Result<[u8; 32], ()> {
    if data.len() > MSG_LEN {
        return Err(());
    }
    let mut write_buffer = [0u8; MSG_LEN + 1];
    write_buffer[1..data.len() + 1].copy_from_slice(data);
    device.write(&write_buffer).unwrap();
    let mut read_buffer = [0u8; MSG_LEN];
    device.read(&mut read_buffer).unwrap();
    Ok(read_buffer)
}

pub fn layer_count(device: &HidDevice) -> u8 {
    write_read(device, &[VialCommand::GetLayerCount.into()]).unwrap()[1]
}

pub fn macro_count(device: &HidDevice) -> u8 {
    write_read(device, &[VialCommand::GetMacroCount.into()]).unwrap()[1]
}

pub fn key_count(payload: &Value) -> usize {
    let layout = &payload["layouts"]["keymap"];
    count_strings(&layout)
}

pub fn get_vial_payload(device: &HidDevice) -> Value {
    //get size
    let size = write_read(
        device,
        &[VialCommand::VialPrefix.into(), VialCommand::GetSize.into()],
    )
    .unwrap();
    let size = LittleEndian::read_u32(&size) as usize;
    //get payload
    let mut payload = Vec::with_capacity(size);
    for block in 0..size.div_ceil(MSG_LEN) {
        let data = write_read(
            device,
            &[
                VialCommand::VialPrefix.into(),
                VialCommand::GetDefinition.into(),
                (block as u8).into(),
            ],
        )
        .unwrap();
        payload.extend_from_slice(&data[0..MSG_LEN.min(size - block * MSG_LEN)]);
    }
    //decompress
    let mut decompressor = XzDecoder::new(&payload[..]);
    let mut payload = String::new();
    decompressor.read_to_string(&mut payload).unwrap();
    serde_json::from_str(&payload).unwrap()
}

pub fn count_strings(value: &Value) -> usize {
    match value {
        serde_json::Value::String(_) => 1,
        serde_json::Value::Array(arr) => arr.iter().map(count_strings).sum(),
        serde_json::Value::Object(obj) => obj.values().map(count_strings).sum(),
        _ => 0,
    }
}