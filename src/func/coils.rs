use crate::coil::{self, CoilError};
use crate::def::{FunctionCode, StatusCode};
use crate::pdu::PDUBuf;
use crate::Instance;

const MAX_READ_QUANTITY: u16 = 2000;

pub fn read_multiple<'a>(
    _inst: &'a Instance,
    coils: &'a [coil::Descriptor<'a>],
    buf: &[u8],
    res: &mut PDUBuf,
) -> StatusCode {
    let [fc, addr_hi, addr_lo, q_hi, q_lo] = match <[u8; 5]>::try_from(buf) {
        Ok(v) => v,
        Err(..) => return StatusCode::IllegalDataValue,
    };

    if fc != FunctionCode::ReadCoils as u8 && fc != FunctionCode::ReadDiscreteInputs as u8 {
        return StatusCode::DeviceFail;
    }

    let start_addr = u16::from_be_bytes([addr_hi, addr_lo]);
    let quantity = u16::from_be_bytes([q_hi, q_lo]);

    // Validate quantity
    if quantity == 0 || quantity > MAX_READ_QUANTITY {
        return StatusCode::IllegalDataValue;
    }

    // If we read multiple coils and one of them don't exist,
    // we just leave it as zero.
    // We don't want to do this if the first coil is missing.
    if coil::find(start_addr, coils).is_none() {
        return StatusCode::IllegalDataAddr;
    }

    let byte_count = (quantity as usize + 7) / 8;
    res.p[1] = byte_count as u8;
    res.size = 2 + byte_count;

    // Clear response buffer
    res.p[2..res.size].fill(0);

    for i in 0..quantity {
        let addr = start_addr + i;
        match coil::find(addr, coils) {
            Some(c) => match c.read() {
                Ok(v) => {
                    if v {
                        res.p[2 + (i as usize / 8)] |= 1 << (i % 8);
                    }
                }
                Err(CoilError::ReadNotSuppported) => (), // Leave as 0
                Err(CoilError::ReadLocked) => return StatusCode::IllegalDataAddr,
                Err(..) => (), // Not relevant for read operations
            },
            None => (), // If coil doesn't exist, it's left as 0
        };
    }

    StatusCode::Ok
}
