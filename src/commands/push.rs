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

    let file_data = fs::read(path)?;
    let file_size = file_data.len();

    // .v4b files have a 16-byte header: "V4BC" + metadata
    // V4-link expects raw bytecode only, so skip the header
    const HEADER_SIZE: usize = 16;

    if file_size < HEADER_SIZE {
        return Err(crate::V4Error::Protocol(
            "File too small to contain V4 bytecode header".to_string(),
        ));
    }

    // Verify magic number "V4BC"
    if &file_data[0..4] != b"V4BC" {
        return Err(crate::V4Error::Protocol(
            "Invalid V4 bytecode file (missing V4BC magic number)".to_string(),
        ));
    }

    // Skip header, send only bytecode
    let bytecode = &file_data[HEADER_SIZE..];
    let size = bytecode.len();

    println!(
        "Loading bytecode from {} ({} bytes bytecode, {} bytes total)...",
        file, size, file_size
    );

    if size == 0 {
        return Err(crate::V4Error::Protocol(
            "Bytecode section is empty".to_string(),
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
    let response = serial.exec(bytecode, timeout)?;

    pb.inc(size as u64);

    if detach {
        pb.finish_with_message("Sent (detached)");
        println!("Bytecode sent to device (not waiting for response)");
        return Ok(());
    }

    pb.finish_with_message("Complete");

    println!("Response: {}", response.error_code.name());

    if response.error_code == ErrorCode::Ok {
        println!("✓ Bytecode deployed successfully");
        if !response.word_indices.is_empty() {
            println!("  Registered {} word(s)", response.word_indices.len());
        }
        Ok(())
    } else {
        Err(crate::V4Error::Device(format!(
            "Device returned error: {}",
            response.error_code.name()
        )))
    }
}
