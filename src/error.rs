use thiserror::Error;

pub type Result<T> = std::result::Result<T, V4Error>;

#[derive(Error, Debug)]
pub enum V4Error {
    #[error("Serial port error: {0}")]
    Serial(#[from] serialport::Error),

    #[error("Protocol error: {0}")]
    Protocol(String),

    #[error("CRC mismatch: expected {expected:#04x}, got {actual:#04x}")]
    CrcMismatch { expected: u8, actual: u8 },

    #[error("Device error: {0}")]
    Device(String),

    #[error("Timeout waiting for response")]
    Timeout,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Compilation error: {0}")]
    Compilation(String),

    #[error("REPL error: {0}")]
    Repl(String),
}
