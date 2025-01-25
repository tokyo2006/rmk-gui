interface Key {
  layer: number; // u8 -> number
  row: number; // u8 -> number
  col: number; // u8 -> number
  position_x: [number, number]; // (f64, f64) -> [number, number]
  position_y: [number, number]; // (f64, f64) -> [number, number]
  width: [number, number]; // (f64, f64) -> [number, number]
  height: [number, number]; // (f64, f64) -> [number, number]
  rotation: number; // f64 -> number
  keycode: number; // KeyCode (u16) -> number
}

interface VialDevice {
  product_string: string; // String -> string
  path: number[]; // CString -> number[]
}
