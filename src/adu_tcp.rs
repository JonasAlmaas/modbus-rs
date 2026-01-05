use byteorder::{BigEndian, ByteOrder};

use crate::pdu;
use crate::Instance;

/// Modbus Application Protocol (MBAP) header
/// - Transaction id (2 bytes BE)
/// - Protocol id (2 bytes BE)
/// - Length (2 bytes BE) (The rest of this buffer; including unit id)
/// - Unit id (1 byte) (Same as Modbus serial slave address)

const MBAP_POS_TRANS_ID: usize = 0;
const MBAP_POS_PROT_ID: usize = 2;
const MBAP_POS_LEN: usize = 4;
const MBAP_POS_UNIT_ID: usize = 6;

pub const MBAP_SIZE: usize = 7;
pub const PROT_ID: u16 = 0;

pub const SIZE_MAX: usize = MBAP_SIZE + pdu::SIZE_MAX;
pub const TCP_PORT: usize = 502;

pub fn handle_req<'a>(inst: &'a Instance<'a>, buf: &[u8], res: &mut [u8; SIZE_MAX]) -> usize {
    if buf.len() < MBAP_SIZE + 1 {
        return 0;
    }

    let transaction_id = BigEndian::read_u16(&buf[MBAP_POS_TRANS_ID..]);
    let protocol_id = BigEndian::read_u16(&buf[MBAP_POS_PROT_ID..]);
    let length: usize = BigEndian::read_u16(&buf[MBAP_POS_LEN..]).into();
    let unit_id = buf[MBAP_POS_UNIT_ID];

    if protocol_id != PROT_ID {
        return 0;
    }

    if length < 1 || length - 1 > pdu::SIZE_MAX || buf.len() < length - 1 + MBAP_SIZE {
        return 0;
    }

    let pdu_size = pdu::handle_req(
        inst,
        &buf[MBAP_SIZE..(MBAP_SIZE + length - 1)],
        (&mut res[MBAP_SIZE..]).try_into().unwrap(),
    );

    if pdu_size == 0 {
        return 0;
    }

    // Build response MBAP
    BigEndian::write_u16(&mut res[MBAP_POS_TRANS_ID..], transaction_id);
    BigEndian::write_u16(&mut res[MBAP_POS_PROT_ID..], protocol_id);
    BigEndian::write_u16(&mut res[MBAP_POS_LEN..], 1 + pdu_size as u16);
    res[MBAP_POS_UNIT_ID] = unit_id;

    MBAP_SIZE + pdu_size
}
