/// V4-link protocol commands
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Command {
    /// Execute bytecode
    Exec = 0x10,
    /// Connection check
    Ping = 0x20,
    /// VM reset
    Reset = 0xFF,
}

/// V4-link protocol error codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ErrorCode {
    /// Success
    Ok = 0x00,
    /// Generic error
    Error = 0x01,
    /// Invalid frame format
    InvalidFrame = 0x02,
    /// Buffer full
    BufferFull = 0x03,
    /// VM execution error
    VmError = 0x04,
}

impl ErrorCode {
    /// Convert u8 to ErrorCode
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0x00 => Some(ErrorCode::Ok),
            0x01 => Some(ErrorCode::Error),
            0x02 => Some(ErrorCode::InvalidFrame),
            0x03 => Some(ErrorCode::BufferFull),
            0x04 => Some(ErrorCode::VmError),
            _ => None,
        }
    }

    /// Get human-readable error name
    pub fn name(&self) -> &'static str {
        match self {
            ErrorCode::Ok => "OK",
            ErrorCode::Error => "ERROR",
            ErrorCode::InvalidFrame => "INVALID_FRAME",
            ErrorCode::BufferFull => "BUFFER_FULL",
            ErrorCode::VmError => "VM_ERROR",
        }
    }
}
