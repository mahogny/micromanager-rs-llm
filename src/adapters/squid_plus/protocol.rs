/// Squid+ microcontroller binary protocol.
///
/// Commands are 8-byte packets; responses are 24-byte packets.
/// Both carry a CRC-8 (CCITT) in the final byte.
///
/// Packet layout (command, 8 bytes):
///   [0] command ID  (auto-incremented, for ACK matching)
///   [1] command code
///   [2..6] payload (command-specific)
///   [7] CRC-8 of bytes 0..6

pub const CMD_LENGTH: usize = 8;
pub const MSG_LENGTH: usize = 24;

// Command codes
pub const CMD_MOVE_W: u8 = 0x04;
pub const CMD_HOME_OR_ZERO: u8 = 0x05;

// Axis identifiers
pub const AXIS_W: u8 = 5;

// Home direction flags
pub const HOME_NEGATIVE: u8 = 1;

// Execution status codes (response byte 1)
pub const STATUS_COMPLETED: u8 = 0x00;
pub const STATUS_IN_PROGRESS: u8 = 0x01;
pub const STATUS_CHECKSUM_ERROR: u8 = 0x02;

/// CRC-8 CCITT lookup table.
const CRC8_TABLE: [u8; 256] = {
    let mut table = [0u8; 256];
    let poly: u8 = 0x07;
    let mut i = 0u16;
    while i < 256 {
        let mut crc = i as u8;
        let mut bit = 0;
        while bit < 8 {
            if crc & 0x80 != 0 {
                crc = (crc << 1) ^ poly;
            } else {
                crc <<= 1;
            }
            bit += 1;
        }
        table[i as usize] = crc;
        i += 1;
    }
    table
};

/// Compute CRC-8 CCITT over the given bytes.
pub fn crc8(data: &[u8]) -> u8 {
    let mut crc: u8 = 0;
    for &b in data {
        crc = CRC8_TABLE[(crc ^ b) as usize];
    }
    crc
}

/// Build a MOVE_W command packet.
///
/// `cmd_id`: rolling command counter (wraps at 256).
/// `usteps`: signed microstep count (positive = forward).
pub fn build_move_w(cmd_id: u8, usteps: i32) -> [u8; CMD_LENGTH] {
    let mut pkt = [0u8; CMD_LENGTH];
    pkt[0] = cmd_id;
    pkt[1] = CMD_MOVE_W;
    let bytes = usteps.to_be_bytes();
    pkt[2] = bytes[0];
    pkt[3] = bytes[1];
    pkt[4] = bytes[2];
    pkt[5] = bytes[3];
    pkt[7] = crc8(&pkt[..7]);
    pkt
}

/// Build a HOME_OR_ZERO command for the W axis.
///
/// Uses HOME_NEGATIVE direction (matching default STAGE_MOVEMENT_SIGN_W = 1).
pub fn build_home_w(cmd_id: u8) -> [u8; CMD_LENGTH] {
    let mut pkt = [0u8; CMD_LENGTH];
    pkt[0] = cmd_id;
    pkt[1] = CMD_HOME_OR_ZERO;
    pkt[2] = AXIS_W;
    pkt[3] = HOME_NEGATIVE;
    pkt[7] = crc8(&pkt[..7]);
    pkt
}

/// Parse a 24-byte response.  Returns `(cmd_id, status)`.
pub fn parse_response(buf: &[u8]) -> Option<(u8, u8)> {
    if buf.len() < MSG_LENGTH {
        return None;
    }
    let expected_crc = crc8(&buf[..MSG_LENGTH - 1]);
    if buf[MSG_LENGTH - 1] != expected_crc {
        return None;
    }
    Some((buf[0], buf[1]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crc8_empty() {
        assert_eq!(crc8(&[]), 0);
    }

    #[test]
    fn move_w_packet_structure() {
        let pkt = build_move_w(42, 12800);
        assert_eq!(pkt[0], 42); // cmd_id
        assert_eq!(pkt[1], CMD_MOVE_W);
        // 12800 = 0x00003200 big-endian
        assert_eq!(pkt[2], 0x00);
        assert_eq!(pkt[3], 0x00);
        assert_eq!(pkt[4], 0x32);
        assert_eq!(pkt[5], 0x00);
        assert_eq!(pkt[7], crc8(&pkt[..7]));
    }

    #[test]
    fn move_w_negative() {
        let pkt = build_move_w(0, -12800);
        let bytes = (-12800i32).to_be_bytes();
        assert_eq!(pkt[2..6], bytes);
    }

    #[test]
    fn home_w_packet() {
        let pkt = build_home_w(7);
        assert_eq!(pkt[0], 7);
        assert_eq!(pkt[1], CMD_HOME_OR_ZERO);
        assert_eq!(pkt[2], AXIS_W);
        assert_eq!(pkt[3], HOME_NEGATIVE);
        assert_eq!(pkt[7], crc8(&pkt[..7]));
    }

    #[test]
    fn parse_valid_response() {
        let mut buf = [0u8; MSG_LENGTH];
        buf[0] = 5; // cmd_id
        buf[1] = STATUS_COMPLETED;
        buf[MSG_LENGTH - 1] = crc8(&buf[..MSG_LENGTH - 1]);
        let (id, status) = parse_response(&buf).unwrap();
        assert_eq!(id, 5);
        assert_eq!(status, STATUS_COMPLETED);
    }

    #[test]
    fn parse_bad_crc() {
        let mut buf = [0u8; MSG_LENGTH];
        buf[0] = 5;
        buf[MSG_LENGTH - 1] = 0xFF; // wrong CRC
        assert!(parse_response(&buf).is_none());
    }
}
