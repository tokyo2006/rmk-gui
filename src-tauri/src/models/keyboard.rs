use super::KeyCode;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug)]
pub struct KeyboardParams {
    pub name:       String,
    pub rows:       u8,
    pub cols:       u8,
    pub layers:     u8,
    pub macros:     u8,
    pub keys:       usize,
    pub payload:    Value,
    pub keymap_set: HashMap<(u8, u8, u8), KeyCode>,
}

impl KeyboardParams {
    pub fn new() -> Self {
        Self {
            name:       String::from(""),
            rows:       0,
            cols:       0,
            layers:     0,
            macros:     0,
            keys:       0,
            payload:    Value::Null,
            keymap_set: HashMap::new(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Key {
    pub position_x: (f64, f64),
    pub position_y: (f64, f64),
    pub width:      (f64, f64),
    pub height:     (f64, f64),
    pub rotation:   f64,
    pub keycode:    KeyCode,
}
