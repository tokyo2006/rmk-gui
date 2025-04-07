use std::mem::discriminant;

use byteorder::{BigEndian, ByteOrder};

use crate::models::*;
use crate::utils::*;

#[tauri::command]
pub async fn get_marcoes(state: tauri::State<'_, AppState>) -> CommandResult<Vec<Vec<MacroAction>>> {
    let state = state.lock().await;
    let device = match state.current_device.as_ref() {
        Some(device) => device,
        None => return Err("Device not connected".to_string()),
    };

    let data = match write_read(device, &[VialCommand::GetMacroBufferSize.into()]) {
        Ok(data) => data,
        Err(e) => return Err(format!("Failed to get macro buffer size: {}", e)),
    };

    let macro_size = BigEndian::read_u16(&data[1..3]) as usize;
    println!("macro size: {}", macro_size);

    let mut macro_memory = Vec::new();
    for i in 0..macro_size.div_ceil(BUFFER_FETCH_CHUNK) {
        let read_size = BUFFER_FETCH_CHUNK.min(macro_size - i * BUFFER_FETCH_CHUNK);
        let mut msg = [0u8; 32];
        msg[0] = VialCommand::GetMacroBuffer.into();
        BigEndian::write_u16(&mut msg[1..=2], (i * BUFFER_FETCH_CHUNK) as u16);
        msg[3] = read_size as u8;

        let data = match write_read(device, &msg) {
            Ok(data) => data,
            Err(e) => return Err(format!("Failed to get macro data, offset {}: {}", i * BUFFER_FETCH_CHUNK, e)),
        };

        macro_memory.extend_from_slice(&data[4..4 + read_size]);
        if macro_memory.iter().filter(|x| **x == 0u8).count() > state.kbd_params.macros as usize {
            break;
        }
    }

    let mut macros: Vec<&[u8]> = macro_memory.split(|x| x == &0u8).collect();
    macros.truncate(state.kbd_params.macros as usize);
    println!("macros: {:?}", macros);

    let deserialized_macros = match macro_deserialize_v2(&macros) {
        Ok(macros) => macros,
        Err(e) => return Err(format!("Failed to deserialize macro data: {}", e)),
    };

    println!("macros: {:?}", deserialized_macros);

    Ok(deserialized_macros)
}

fn macro_deserialize_v2(raw_macros: &Vec<&[u8]>) -> Result<Vec<Vec<MacroAction>>, String> {
    let mut macros_actions = Vec::new();
    for (idx, raw_macro) in raw_macros.iter().enumerate() {
        let mut macro_actions = Vec::new();
        let mut raw_macro = Vec::from(*raw_macro);
        let mut action: Option<MacroAction> = None;

        while !raw_macro.is_empty() {
            let mut code = raw_macro[0];
            let mut macro_code = MacroCode::from(code);

            if let MacroCode::Prefix = macro_code {
                // Down Up Tap Delay
                raw_macro.remove(0);

                if raw_macro.is_empty() {
                    return Err(format!("Macro format error: insufficient data after prefix, index: {}", idx));
                }

                code = raw_macro.remove(0);
                macro_code = MacroCode::from(code);

                if action.is_none() {
                    action = Some(MacroAction::from(macro_code));
                }

                if discriminant(action.as_ref().unwrap()) != discriminant(&MacroAction::from(macro_code)) {
                    macro_actions.push(action.take().unwrap());
                    action = Some(MacroAction::from(macro_code));
                }

                match action.as_mut().unwrap() {
                    MacroAction::Down(v) | MacroAction::Up(v) | MacroAction::Tap(v) => {
                        if raw_macro.is_empty() {
                            return Err(format!("Macro format error: missing keycode, index: {}", idx));
                        }

                        let key = KeyCode::from(&[0, raw_macro.remove(0)][..]);
                        v.push(key);
                    }
                    MacroAction::Delay(x) => {
                        if raw_macro.len() < 2 {
                            return Err(format!("Macro format error: incomplete delay data, index: {}", idx));
                        }

                        let delay = (raw_macro.remove(0), raw_macro.remove(0));
                        let delay = (delay.0 - 1) as u16 + (delay.1 - 1) as u16 * 255;
                        *x = Some(delay);
                    }
                    _ => {}
                }
            } else {
                // Text
                if action.is_none() {
                    action = Some(MacroAction::Text("".to_string()));
                }

                if discriminant(action.as_ref().unwrap()) != discriminant(&MacroAction::Text("".to_string())) {
                    macro_actions.push(action.take().unwrap());
                    action = Some(MacroAction::Text("".to_string()));
                }

                if let MacroAction::Text(v) = action.as_mut().unwrap() {
                    v.push(raw_macro.remove(0) as char);
                }
            }
        }

        if let Some(action) = action {
            macro_actions.push(action);
        }

        macros_actions.push(macro_actions);
    }

    Ok(macros_actions)
}
