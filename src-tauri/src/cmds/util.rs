use crate::models::*;
use crate::utils::*;

#[tauri::command]
pub async fn get_key_from_keycode(_state: tauri::State<'_, AppState>, keycode: u16) -> CommandResult<Key> {
    let keycode = keycode.to_be_bytes();
    Ok(Key::new(
        (0, 0, 0),
        (0f64, 0f64),
        (0f64, 0f64),
        (1f64, 1f64),
        (1f64, 1f64),
        (0f64, 0f64, 0f64),
        (&keycode)[..].into(),
    ))
}
