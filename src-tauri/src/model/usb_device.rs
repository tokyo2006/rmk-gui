// 

use std::ffi::CString;
use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct UsbDevice {
    pub product_string: String,
    pub path: CString,
    pub vendor_id: u16,
    pub product_id: u16,
}

impl UsbDevice {
    pub fn new(product_string: String, path: CString, vendor_id: u16, product_id: u16) -> Self {
        UsbDevice {
            product_string,
            path,
            vendor_id,
            product_id,
        }
    }
}