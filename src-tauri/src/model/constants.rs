pub const VIAL_SERIAL_NUMBER_MAGIC: &str = "vial:f64c2b3c";
// 也许叫这个名字？
pub const VIAL_USAGE_PAGE_MAGIC: u16 = 0xFF60;
pub const VIAL_USAGE_MAGIC: u16 = 0x61;

#[repr(u8)]
pub enum VialCommand {
    GetProtocolVersion = 0x01,
    GetKeyboardValue = 0x02,
    SetKeyboardValue = 0x03,
    GetKeycode = 0x04,
    SetKeycode = 0x05,
    LightingSetValue = 0x07,
    LightingGetValue = 0x08,
    LightingSave = 0x09,
    MacroGetCount = 0x0C,
    MacroGetBufferSize = 0x0D,
    MacroGetBuffer = 0x0E,
    MacroSetBuffer = 0x0F,
    GetLayerCount = 0x11,
    KeymapGetBuffer = 0x12,
    VialPrefix = 0xFE,
}

