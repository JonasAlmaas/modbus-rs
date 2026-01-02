use crate::mbdef::{FunctionCode, StatusCode};
use crate::mbpdu::PDUBuf;

pub fn read_multiple(buf: &[u8], res: &mut PDUBuf) -> StatusCode {
    let [fc, addr_hi, addr_lo, q_hi, q_lo] = match <[u8; 5]>::try_from(buf) {
        Ok(v) => v,
        Err(..) => return StatusCode::IllegalDataValue,
    };

    if fc != FunctionCode::ReadInputRegs as u8 && fc != FunctionCode::ReadHoldingRegs as u8 {
        return StatusCode::DeviceFail;
    }

    let _start_addr = u16::from_be_bytes([addr_hi, addr_lo]);
    let quantity = u16::from_be_bytes([q_hi, q_lo]);

    if quantity == 0 || quantity > 125 {
        // MBREG_N_READ_MAX
        return StatusCode::IllegalDataValue;
    }

    res.p[1] = (2 * quantity) as u8; // Byte count
    res.size = 2;

    // TODO: Do this properly

    for i in 0..quantity {
        let j = (2 + i * 2) as usize;
        if i % 2 == 0 {
            res.p[j] = 0xDE;
            res.p[j + 1] = 0xAD;
        } else {
            res.p[j] = 0xBE;
            res.p[j + 1] = 0xEF;
        }
        res.size += 2;
    }

    StatusCode::Ok
}
