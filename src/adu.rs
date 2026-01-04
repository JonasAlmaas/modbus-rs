use crate::{crc, pdu, Instance};

/// The minimum size of a valid Modbus ADU buffer
///
/// - 1 Slave address
/// - 1 Function code (Technically not part of the ADU, but required)
/// - 2 crc
const SIZE_MIN: usize = 4;

/// The maximum size of a valid Modbus ADU buffer
///
/// - 1 Slave address
/// - 253 PDU
///   - 1 Function code
///   - 252 PDU data
/// - 2 crc
pub const SIZE_MAX: usize = 256;

pub const SLAVE_ADDR_MIN: u8 = 1;
pub const SLAVE_ADDR_MAX: u8 = 247;

const SLAVE_ADDR_BROADCAST: u8 = 0;
const SLAVE_ADDR_DEFAULT_RESP: u8 = 248;

fn prep_res(slave_addr: u8, res: &mut [u8; SIZE_MAX], pdu_size: usize) -> usize {
    res[0] = slave_addr;
    let res_size = 1 + pdu_size;

    let crc = crc::crc16(&res[..res_size]);
    let crc = crc.to_le_bytes();
    res[res_size] = crc[0];
    res[res_size + 1] = crc[1];

    res_size + 2
}

pub fn handle_req<'a>(inst: &'a Instance<'a>, buf: &[u8], res: &mut [u8; SIZE_MAX]) -> usize {
    if !inst.serial.is_some() || buf.len() < SIZE_MIN {
        return 0;
    }

    let recv_crc = u16::from_le_bytes(buf[(buf.len() - 2)..].try_into().unwrap());
    if recv_crc != crc::crc16(&buf[..buf.len() - 2]) {
        return 0;
    }

    let recv_slave_addr = buf[0];
    if !match recv_slave_addr {
        x if x == inst.serial.as_ref().unwrap().slave_addr => true,
        x if x == SLAVE_ADDR_BROADCAST => true,
        x if x == SLAVE_ADDR_DEFAULT_RESP => true,
        _ => false,
    } {
        return 0;
    }

    let pdu_size = pdu::handle_req(
        inst,
        &buf[1..buf.len() - 2],
        (&mut res[1..1 + pdu::SIZE_MAX]).try_into().unwrap(),
    );
    if pdu_size == 0 || recv_slave_addr == SLAVE_ADDR_DEFAULT_RESP {
        return 0;
    }

    prep_res(recv_slave_addr, res, pdu_size)
}
