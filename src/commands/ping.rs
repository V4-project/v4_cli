use crate::Result;
use crate::protocol::ErrorCode;
use crate::serial::V4Serial;
use std::time::Duration;

/// Send PING command to device
pub fn ping(port: &str, timeout: Duration) -> Result<()> {
    let mut serial = V4Serial::open_default(port)?;

    println!("Sending PING to {}...", port);

    let err_code = serial.ping(timeout)?;

    println!("Response: {}", err_code.name());

    if err_code == ErrorCode::Ok {
        println!("✓ Device is responding");
        Ok(())
    } else {
        Err(crate::V4Error::Device(format!(
            "Device returned error: {}",
            err_code.name()
        )))
    }
}
