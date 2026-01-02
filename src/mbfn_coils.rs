use crate::mbdef::{FunctionCode, StatusCode};
use crate::mbpdu::PDUBuf;

pub fn read_multiple(buf: &[u8], res: &mut PDUBuf) -> StatusCode {
    let [fc, addr_hi, addr_lo, q_hi, q_lo] = match <[u8; 5]>::try_from(buf) {
        Ok(v) => v,
        Err(..) => return StatusCode::IllegalDataValue,
    };

    if fc != FunctionCode::ReadCoils as u8
    /*&& fc != FunctionCode::ReadDiscreteInputs as u8*/
    {
        return StatusCode::DeviceFail;
    }

    let start_addr = u16::from_be_bytes([addr_hi, addr_lo]);
    let quantity = u16::from_be_bytes([q_hi, q_lo]);

    if quantity == 0 || quantity > 2000
    /* MBCOIL_N_READ_MAX */
    {
        return StatusCode::IllegalDataValue;
    }

    // TODO: Find descriptor?

    let byte_count: u8 = ((quantity + 7) / 8) as u8;
    res.p[1] = byte_count;
    res.size = 0x2 + (byte_count as usize);

    // TODO: Clear response buffer
    for i in &mut res.p[2..(2 + (byte_count as usize))] {
        *i = 0;
    }

    for i in 0..quantity {
        let _addr = start_addr + i;
        // TODO: Find descriptor
        // TODO: Write bit

        // If active, set the bit
        res.p[2 + (i as usize / 8)] |= 1 << (i % 8);
    }

    StatusCode::Ok
}
