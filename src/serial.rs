use crate::protocol::{Command, ErrorCode, Frame};
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
        self.port.write_all(&encoded)?;
        self.port.flush()?;
        Ok(())
    }

    /// Receive response with timeout
    pub fn recv_response(&mut self, timeout: Duration) -> Result<Vec<u8>> {
        let start = Instant::now();
        let mut response = Vec::new();

        // Response is always 5 bytes: [STX][LEN_L][LEN_H][ERR_CODE][CRC]
        while response.len() < 5 && start.elapsed() < timeout {
            let available = self.port.bytes_to_read()? as usize;
            if available > 0 {
                let mut buf = vec![0u8; available];
                let n = self.port.read(&mut buf)?;
                response.extend_from_slice(&buf[..n]);
            }
            std::thread::sleep(Duration::from_millis(10));
        }

        if response.len() < 5 {
            return Err(V4Error::Timeout);
        }

        Ok(response)
    }

    /// Send command and wait for response
    pub fn send_command(
        &mut self,
        command: Command,
        payload: &[u8],
        timeout: Duration,
    ) -> Result<ErrorCode> {
        let frame = Frame::new(command, payload.to_vec())?;
        self.send_frame(&frame)?;

        let response = self.recv_response(timeout)?;
        Frame::decode_response(&response)
    }

    /// Send PING command
    pub fn ping(&mut self, timeout: Duration) -> Result<ErrorCode> {
        self.send_command(Command::Ping, &[], timeout)
    }

    /// Send RESET command
    pub fn reset(&mut self, timeout: Duration) -> Result<ErrorCode> {
        self.send_command(Command::Reset, &[], timeout)
    }

    /// Send EXEC command with bytecode
    pub fn exec(&mut self, bytecode: &[u8], timeout: Duration) -> Result<ErrorCode> {
        self.send_command(Command::Exec, bytecode, timeout)
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
