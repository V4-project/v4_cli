use crate::protocol::ErrorCode;
use crate::serial::V4Serial;
use crate::Result;
use std::time::Duration;

/// Send RESET command to device
pub fn reset(port: &str, timeout: Duration) -> Result<()> {
    let mut serial = V4Serial::open_default(port)?;

    println!("Sending RESET to {}...", port);

    let err_code = serial.reset(timeout)?;

    println!("Response: {}", err_code.name());

    if err_code == ErrorCode::Ok {
        println!("✓ VM reset successful");
        Ok(())
    } else {
        Err(crate::V4Error::Device(format!(
            "Device returned error: {}",
            err_code.name()
        )))
    }
}
