use strum::EnumIter;

use super::KeyCode;

#[derive(Debug, Clone)]
pub enum MacroAction {
    Text(String),
    Tap(Vec<KeyCode>),
    Down(Vec<KeyCode>),
    Up(Vec<KeyCode>),
    Delay(Option<u16>),
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, EnumIter, PartialEq, Eq)]
pub enum MacroCode {
    /// 特殊动作前缀，所有控制序列必须以该字节开头
    Prefix  = 1,
    /// 单字节按键点击 (SS_TAP_CODE)
    Tap     = 0,
    /// 单字节按键按下 (SS_DOWN_CODE)
    Down    = 2,
    /// 单字节按键释放 (SS_UP_CODE)
    Up      = 3,
    /// 延迟控制码 (SS_DELAY_CODE)
    Delay   = 4,
    /// 双字节扩展点击 (VIAL_MACRO_EXT_TAP)
    ExtTap  = 6,
    /// 双字节扩展按下 (VIAL_MACRO_EXT_DOWN)
    ExtDown = 7,
    /// 双字节扩展释放 (VIAL_MACRO_EXT_UP)
    ExtUp   = 8,
    /// 文本输入 (VIAL_MACRO_TEXT)
    Text    = 9,
}

impl From<u8> for MacroCode {
    fn from(value: u8) -> Self {
        match value {
            0 => MacroCode::Tap,
            1 => MacroCode::Prefix,
            2 => MacroCode::Down,
            3 => MacroCode::Up,
            4 => MacroCode::Delay,
            // 6 => MacroCode::ExtTap,
            // 7 => MacroCode::ExtDown,
            // 8 => MacroCode::ExtUp,
            _ => MacroCode::Text,
        }
    }
}

impl From<MacroCode> for MacroAction {
    fn from(value: MacroCode) -> Self {
        match value {
            MacroCode::Prefix => MacroAction::Tap(vec![]),
            MacroCode::Tap => MacroAction::Tap(vec![]),
            MacroCode::Down => MacroAction::Down(vec![]),
            MacroCode::Up => MacroAction::Up(vec![]),
            MacroCode::Delay => MacroAction::Delay(None),
            MacroCode::Text => MacroAction::Text("".to_string()),
            _ => panic!("not support"),
        }
    }
}
