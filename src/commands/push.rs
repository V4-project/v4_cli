use crate::Result;
use crate::protocol::ErrorCode;
use crate::serial::V4Serial;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::path::Path;
use std::time::Duration;

/// Push bytecode to device
pub fn push(file: &str, port: &str, detach: bool, timeout: Duration) -> Result<()> {
    // Read bytecode file
    let path = Path::new(file);
    if !path.exists() {
        return Err(crate::V4Error::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Bytecode file not found: {}", file),
        )));
    }

    let bytecode = fs::read(path)?;
    let size = bytecode.len();

    println!("Loading bytecode from {} ({} bytes)...", file, size);

    if size == 0 {
        return Err(crate::V4Error::Protocol(
            "Bytecode file is empty".to_string(),
        ));
    }

    // Create progress bar
    let pb = ProgressBar::new(size as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {bytes}/{total_bytes} {msg}")
            .unwrap()
            .progress_chars("=>-"),
    );

    // Open serial port
    let mut serial = V4Serial::open_default(port)?;

    pb.set_message("Sending...");

    // Send EXEC command
    let err_code = serial.exec(&bytecode, timeout)?;

    pb.inc(size as u64);

    if detach {
        pb.finish_with_message("Sent (detached)");
        println!("Bytecode sent to device (not waiting for response)");
        return Ok(());
    }

    pb.finish_with_message("Complete");

    println!("Response: {}", err_code.name());

    if err_code == ErrorCode::Ok {
        println!("✓ Bytecode deployed successfully");
        Ok(())
    } else {
        Err(crate::V4Error::Device(format!(
            "Device returned error: {}",
            err_code.name()
        )))
    }
}
