// Protocol types - to be implemented
#[derive(Debug, Clone, Copy)]
pub enum Command {
    Exec = 0x10,
    Ping = 0x20,
    Reset = 0xFF,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    Ok = 0x00,
    Error = 0x01,
    InvalidFrame = 0x02,
    BufferFull = 0x03,
    VmError = 0x04,
}
