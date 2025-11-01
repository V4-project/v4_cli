use super::calc_crc8;
use super::types::{Command, ErrorCode};
use crate::{Result, V4Error};

/// V4-link protocol start marker
const STX: u8 = 0xA5;

/// Maximum payload size (512 bytes)
const MAX_PAYLOAD_SIZE: usize = 512;

/// V4-link frame
///
/// Format: [STX][LEN_L][LEN_H][CMD][DATA...][CRC8]
/// - STX: 0xA5
/// - LEN_L, LEN_H: Payload length (little-endian u16)
/// - CMD: Command code
/// - DATA: Payload (0-512 bytes)
/// - CRC8: Checksum over [LEN_L][LEN_H][CMD][DATA...]
#[derive(Debug, Clone)]
pub struct Frame {
    pub command: Command,
    pub payload: Vec<u8>,
}

impl Frame {
    /// Create a new frame
    pub fn new(command: Command, payload: Vec<u8>) -> Result<Self> {
        if payload.len() > MAX_PAYLOAD_SIZE {
            return Err(V4Error::Protocol(format!(
                "Payload too large: {} bytes (max {})",
                payload.len(),
                MAX_PAYLOAD_SIZE
            )));
        }
        Ok(Self { command, payload })
    }

    /// Encode frame to bytes
    pub fn encode(&self) -> Vec<u8> {
        let length = self.payload.len() as u16;
        let mut frame = Vec::with_capacity(5 + self.payload.len());

        // STX
        frame.push(STX);

        // Length (little-endian)
        frame.push((length & 0xFF) as u8);
        frame.push(((length >> 8) & 0xFF) as u8);

        // Command
        frame.push(self.command as u8);

        // Payload
        frame.extend_from_slice(&self.payload);

        // CRC8 over everything except STX
        let crc = calc_crc8(&frame[1..]);
        frame.push(crc);

        frame
    }

    /// Decode response frame
    ///
    /// Response format: [STX][0x01][0x00][ERR_CODE][CRC8]
    pub fn decode_response(data: &[u8]) -> Result<ErrorCode> {
        if data.len() < 5 {
            return Err(V4Error::Protocol(format!(
                "Response too short: {} bytes (expected 5)",
                data.len()
            )));
        }

        if data[0] != STX {
            return Err(V4Error::Protocol(format!(
                "Invalid STX: {:#04x} (expected {:#04x})",
                data[0], STX
            )));
        }

        let length = u16::from_le_bytes([data[1], data[2]]);
        if length != 1 {
            return Err(V4Error::Protocol(format!(
                "Invalid response length: {} (expected 1)",
                length
            )));
        }

        let err_code = data[3];
        let expected_crc = calc_crc8(&data[1..4]);
        let actual_crc = data[4];

        if expected_crc != actual_crc {
            return Err(V4Error::CrcMismatch {
                expected: expected_crc,
                actual: actual_crc,
            });
        }

        ErrorCode::from_u8(err_code)
            .ok_or_else(|| V4Error::Protocol(format!("Unknown error code: {:#04x}", err_code)))
    }
}

/// Builder for creating frames
pub struct FrameBuilder {
    command: Command,
    payload: Vec<u8>,
}

impl FrameBuilder {
    pub fn new(command: Command) -> Self {
        Self {
            command,
            payload: Vec::new(),
        }
    }

    pub fn payload(mut self, payload: Vec<u8>) -> Self {
        self.payload = payload;
        self
    }

    pub fn build(self) -> Result<Frame> {
        Frame::new(self.command, self.payload)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ping_frame_encoding() {
        let frame = Frame::new(Command::Ping, vec![]).unwrap();
        let encoded = frame.encode();

        // [STX][0x00][0x00][0x20][CRC]
        assert_eq!(encoded[0], 0xA5); // STX
        assert_eq!(encoded[1], 0x00); // LEN_L
        assert_eq!(encoded[2], 0x00); // LEN_H
        assert_eq!(encoded[3], 0x20); // CMD_PING
        assert_eq!(encoded.len(), 5); // STX + LEN + CMD + CRC

        // Verify CRC
        let expected_crc = calc_crc8(&[0x00, 0x00, 0x20]);
        assert_eq!(encoded[4], expected_crc);
    }

    #[test]
    fn test_exec_frame_with_payload() {
        let payload = vec![0x42, 0x43];
        let frame = Frame::new(Command::Exec, payload.clone()).unwrap();
        let encoded = frame.encode();

        // [STX][0x02][0x00][0x10][0x42][0x43][CRC]
        assert_eq!(encoded[0], 0xA5); // STX
        assert_eq!(encoded[1], 0x02); // LEN_L
        assert_eq!(encoded[2], 0x00); // LEN_H
        assert_eq!(encoded[3], 0x10); // CMD_EXEC
        assert_eq!(encoded[4], 0x42); // Payload
        assert_eq!(encoded[5], 0x43);
        assert_eq!(encoded.len(), 7);

        // Verify CRC over [LEN_L][LEN_H][CMD][DATA...]
        let expected_crc = calc_crc8(&[0x02, 0x00, 0x10, 0x42, 0x43]);
        assert_eq!(encoded[6], expected_crc);
    }

    #[test]
    fn test_response_decode_ok() {
        // [STX][LEN_L=0x01][LEN_H=0x00][ERR_OK=0x00][CRC]
        let response_data = vec![0x01, 0x00, 0x00]; // Length=1, ErrorCode=OK
        let crc = calc_crc8(&response_data);
        let mut response = vec![0xA5];
        response.extend_from_slice(&response_data);
        response.push(crc);

        let err_code = Frame::decode_response(&response).unwrap();
        assert_eq!(err_code, ErrorCode::Ok);
    }

    #[test]
    fn test_response_decode_error() {
        // [STX][LEN_L=0x01][LEN_H=0x00][ERR_ERROR=0x01][CRC]
        let response_data = vec![0x01, 0x00, 0x01]; // Length=1, ErrorCode=Error
        let crc = calc_crc8(&response_data);
        let mut response = vec![0xA5];
        response.extend_from_slice(&response_data);
        response.push(crc);

        let err_code = Frame::decode_response(&response).unwrap();
        assert_eq!(err_code, ErrorCode::Error);
    }

    #[test]
    fn test_response_decode_crc_mismatch() {
        // Invalid CRC
        let response = vec![0xA5, 0x01, 0x00, 0x00, 0xFF];
        let result = Frame::decode_response(&response);
        assert!(matches!(result, Err(V4Error::CrcMismatch { .. })));
    }

    #[test]
    fn test_payload_too_large() {
        let payload = vec![0; MAX_PAYLOAD_SIZE + 1];
        let result = Frame::new(Command::Exec, payload);
        assert!(matches!(result, Err(V4Error::Protocol(_))));
    }

    #[test]
    fn test_frame_builder() {
        let frame = FrameBuilder::new(Command::Reset)
            .payload(vec![])
            .build()
            .unwrap();

        assert_eq!(frame.command as u8, 0xFF);
        assert_eq!(frame.payload.len(), 0);
    }
}
