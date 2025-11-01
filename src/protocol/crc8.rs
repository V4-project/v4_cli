/// Calculate CRC-8 checksum
///
/// Polynomial: 0x07
/// Initial value: 0x00
///
/// # Examples
///
/// ```
/// use v4_cli::protocol::calc_crc8;
///
/// let data = b"123456789";
/// assert_eq!(calc_crc8(data), 0xF4);
/// ```
pub fn calc_crc8(data: &[u8]) -> u8 {
    let mut crc = 0u8;
    for &byte in data {
        crc ^= byte;
        for _ in 0..8 {
            if crc & 0x80 != 0 {
                crc = (crc << 1) ^ 0x07;
            } else {
                crc <<= 1;
            }
        }
    }
    crc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crc8_reference() {
        // Reference test case from specification
        let data = b"123456789";
        assert_eq!(calc_crc8(data), 0xF4);
    }

    #[test]
    fn test_crc8_empty() {
        assert_eq!(calc_crc8(&[]), 0x00);
    }

    #[test]
    fn test_crc8_single_byte() {
        assert_eq!(calc_crc8(&[0x00]), 0x00);
        assert_eq!(calc_crc8(&[0xFF]), 0xF3);
    }

    #[test]
    fn test_crc8_protocol_frame() {
        // Test with actual protocol frame data
        // [LEN_L][LEN_H][CMD][DATA...]
        let frame_data = [0x00, 0x00, 0x20]; // Ping command, no data
        let crc = calc_crc8(&frame_data);
        assert_eq!(crc, 0xE0);
    }
}
