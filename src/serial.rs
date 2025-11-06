use crate::protocol::{Command, ErrorCode, Frame, Response};
use crate::{Result, V4Error};
use serialport::SerialPort;
use std::time::{Duration, Instant};

/// Default baud rate for V4-link protocol
pub const DEFAULT_BAUD_RATE: u32 = 115200;

/// V4 Serial port wrapper
pub struct V4Serial {
    port: Box<dyn SerialPort>,
}

impl V4Serial {
    /// Open a serial port
    pub fn open(path: &str, baud_rate: u32) -> Result<Self> {
        let port = serialport::new(path, baud_rate)
            .timeout(Duration::from_secs(5))
            .open()?;

        Ok(Self { port })
    }

    /// Open with default baud rate
    pub fn open_default(path: &str) -> Result<Self> {
        Self::open(path, DEFAULT_BAUD_RATE)
    }

    /// Send a frame
    pub fn send_frame(&mut self, frame: &Frame) -> Result<()> {
        let encoded = frame.encode();
        eprintln!("DEBUG: Sending frame ({} bytes): {:02X?}", encoded.len(), encoded);
        self.port.write_all(&encoded)?;
        self.port.flush()?;
        Ok(())
    }

    /// Receive response with timeout
    pub fn recv_response(&mut self, timeout: Duration) -> Result<Vec<u8>> {
        const STX: u8 = 0xA5;
        let start = Instant::now();
        let mut buffer = Vec::new();

        // Read bytes until we find STX or timeout
        while start.elapsed() < timeout {
            let available = self.port.bytes_to_read()? as usize;
            if available > 0 {
                let mut buf = vec![0u8; available];
                let n = self.port.read(&mut buf)?;
                buffer.extend_from_slice(&buf[..n]);

                // Search for STX
                if let Some(pos) = buffer.iter().position(|&b| b == STX) {
                    // Found STX, need to read header first to get frame length
                    let mut response = vec![STX];
                    let mut remaining_start = pos + 1;

                    // Read at least 4 bytes to get LEN field: STX + LEN_L + LEN_H + ERR_CODE
                    while response.len() < 4 && start.elapsed() < timeout {
                        if remaining_start < buffer.len() {
                            let to_copy =
                                std::cmp::min(4 - response.len(), buffer.len() - remaining_start);
                            response.extend_from_slice(
                                &buffer[remaining_start..remaining_start + to_copy],
                            );
                            remaining_start += to_copy;
                        } else {
                            // Need to read more data
                            let available = self.port.bytes_to_read()? as usize;
                            if available > 0 {
                                let mut buf = vec![0u8; available];
                                let n = self.port.read(&mut buf)?;
                                buffer.extend_from_slice(&buf[..n]);
                            } else {
                                std::thread::sleep(Duration::from_millis(20));
                            }
                        }
                    }

                    if response.len() >= 4 {
                        // Parse length field to determine total frame size
                        let payload_len = u16::from_le_bytes([response[1], response[2]]) as usize;
                        let total_frame_len = 1 + 2 + payload_len + 1; // STX + LEN(2) + PAYLOAD + CRC

                        // Continue reading until we have the complete frame
                        while response.len() < total_frame_len && start.elapsed() < timeout {
                            if remaining_start < buffer.len() {
                                let to_copy = std::cmp::min(
                                    total_frame_len - response.len(),
                                    buffer.len() - remaining_start,
                                );
                                response.extend_from_slice(
                                    &buffer[remaining_start..remaining_start + to_copy],
                                );
                                remaining_start += to_copy;
                            } else {
                                // Need to read more data
                                let available = self.port.bytes_to_read()? as usize;
                                if available > 0 {
                                    let mut buf = vec![0u8; available];
                                    let n = self.port.read(&mut buf)?;
                                    buffer.extend_from_slice(&buf[..n]);
                                } else {
                                    std::thread::sleep(Duration::from_millis(20));
                                }
                            }
                        }

                        if response.len() == total_frame_len {
                            eprintln!("DEBUG: Received complete frame ({} bytes): {:02X?}", response.len(), response);
                            return Ok(response);
                        }
                    }
                }
            }
            std::thread::sleep(Duration::from_millis(20));
        }

        Err(V4Error::Timeout)
    }

    /// Send command and wait for response
    pub fn send_command(
        &mut self,
        command: Command,
        payload: &[u8],
        timeout: Duration,
    ) -> Result<Response> {
        let frame = Frame::new(command, payload.to_vec())?;
        self.send_frame(&frame)?;

        let response = self.recv_response(timeout)?;
        Frame::decode_response(&response)
    }

    /// Send PING command
    pub fn ping(&mut self, timeout: Duration) -> Result<ErrorCode> {
        Ok(self.send_command(Command::Ping, &[], timeout)?.error_code)
    }

    /// Send RESET command
    pub fn reset(&mut self, timeout: Duration) -> Result<ErrorCode> {
        Ok(self.send_command(Command::Reset, &[], timeout)?.error_code)
    }

    /// Send EXEC command with bytecode
    pub fn exec(&mut self, bytecode: &[u8], timeout: Duration) -> Result<Response> {
        self.send_command(Command::Exec, bytecode, timeout)
    }

    /// Query stack state (data stack + return stack)
    pub fn query_stack(&mut self, timeout: Duration) -> Result<Response> {
        self.send_command(Command::QueryStack, &[], timeout)
    }

    /// Query memory dump at address
    pub fn query_memory(&mut self, addr: u32, len: u16, timeout: Duration) -> Result<Response> {
        let mut payload = Vec::with_capacity(6);
        // Address (little-endian u32)
        payload.extend_from_slice(&addr.to_le_bytes());
        // Length (little-endian u16)
        payload.extend_from_slice(&len.to_le_bytes());
        self.send_command(Command::QueryMemory, &payload, timeout)
    }

    /// Query word information by index
    pub fn query_word(&mut self, word_idx: u16, timeout: Duration) -> Result<Response> {
        let payload = word_idx.to_le_bytes();
        self.send_command(Command::QueryWord, &payload, timeout)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_baud_rate() {
        assert_eq!(DEFAULT_BAUD_RATE, 115200);
    }
}
