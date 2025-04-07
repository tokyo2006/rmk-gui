use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("HID device error: {0}")]
    HidError(#[from] hidapi::HidError),

    #[error("Device not found")]
    DeviceNotFound,

    #[error("Invalid device path")]
    InvalidDevicePath,

    #[error("Serial communication error: {0}")]
    SerialError(String),

    #[error("JSON parse error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Permission denied")]
    PermissionDenied,

    #[error("Device not connected")]
    DeviceNotConnected,

    #[error("Unsupported operation")]
    UnsupportedOperation,

    #[error("Unknown error: {0}")]
    Unknown(String),
}

// Use Result<T, String> for Tauri commands
// This is because complex error types don't implement Serialize required by Tauri
pub type CommandResult<T> = Result<T, String>;
