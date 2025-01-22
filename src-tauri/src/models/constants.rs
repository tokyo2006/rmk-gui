pub const VIAL_SERIAL_NUMBER_MAGIC: &str = "vial:f64c2b3c";
// 也许叫这个名字？
pub const VIAL_USAGE_PAGE_MAGIC: u16 = 0xFF60;
pub const VIAL_USAGE_MAGIC: u16 = 0x61;

pub const MSG_LEN: usize = 32;
pub const BUFFER_FETCH_CHUNK: usize = 28;

#[repr(u8)]
pub enum VialCommand {
    GetSize         = 0x01,
    GetDefinition   = 0x02,
    GetMacroCount   = 0x0C,
    GetLayerCount   = 0x11,
    GetKeymapBuffer = 0x12,
    VialPrefix      = 0xFE,
}

impl Into<u8> for VialCommand {
    fn into(self) -> u8 {
        self as u8
    }
}
