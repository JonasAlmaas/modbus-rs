use std::u16;

use byteorder::{BigEndian, ByteOrder};

use crate::coil::{self, Error};
use crate::def::{FunctionCode, StatusCode};
use crate::pdu::PDUBuf;
use crate::Instance;

const COIL_ON: u16 = 0xFFFF;
const COIL_OFF: u16 = 0x0000;

const MAX_READ_QUANTITY: u16 = 0x07D0;
const MAX_WRITE_QUANTITY: u16 = 0x07B0;

pub fn read_multiple(
    inst: &Instance,
    buf: &[u8],
    res: &mut PDUBuf,
    read_disc_inputs: bool,
) -> Result<StatusCode, ()> {
    let coils = if read_disc_inputs {
        match &inst.disc_inputs {
            Some(c) => c,
            None => return Err(()),
        }
    } else {
        match &inst.coils {
            Some(c) => c,
            None => return Err(()),
        }
    };

    let [fc, addr_hi, addr_lo, q_hi, q_lo] = match <[u8; 5]>::try_from(buf) {
        Ok(v) => v,
        Err(..) => return Ok(StatusCode::IllegalDataValue),
    };

    if fc != FunctionCode::ReadCoils as u8 && fc != FunctionCode::ReadDiscreteInputs as u8 {
        return Ok(StatusCode::DeviceFail);
    }

    let start_addr = u16::from_be_bytes([addr_hi, addr_lo]);
    let quantity = u16::from_be_bytes([q_hi, q_lo]);

    // Validate quantity
    if quantity == 0 || quantity > MAX_READ_QUANTITY {
        return Ok(StatusCode::IllegalDataValue);
    }

    // If we read multiple coils and one of them don't exist,
    // we just leave it as zero.
    // We don't want to do this if the first coil is missing.
    if coil::find(start_addr, coils).is_none() {
        return Ok(StatusCode::IllegalDataAddr);
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
                Err(Error::ReadNotSuppported) => (), // Leave as 0
                Err(Error::ReadLocked) => return Ok(StatusCode::IllegalDataAddr),
                Err(..) => return Ok(StatusCode::DeviceFail),
            },
            None => (), // If coil doesn't exist, it's left as 0
        };
    }

    Ok(StatusCode::Ok)
}

pub fn write_single<'a>(
    inst: &'a Instance<'a>,
    buf: &[u8],
    res: &mut PDUBuf,
) -> Result<StatusCode, ()> {
    let coils = match &inst.coils {
        Some(c) => c,
        None => return Err(()),
    };

    let [fc, addr_hi, addr_lo, v_hi, v_lo] = match <[u8; 5]>::try_from(buf) {
        Ok(v) => v,
        Err(..) => return Ok(StatusCode::IllegalDataValue),
    };

    if fc != FunctionCode::WriteSingleReg as u8 {
        return Ok(StatusCode::DeviceFail);
    }

    let addr = u16::from_be_bytes([addr_hi, addr_lo]);
    let value = u16::from_be_bytes([v_hi, v_lo]);

    // Validate coil value (must be 0x0000 or 0xFF00)
    if !matches!(value, COIL_ON | COIL_OFF) {
        return Ok(StatusCode::IllegalDataValue);
    }

    match coil::find(addr, coils) {
        Some(c) => match c.write(value == COIL_ON) {
            Ok(()) => (),
            Err(Error::WriteNotSuppported) => return Ok(StatusCode::IllegalDataAddr),
            Err(Error::WriteLocked) => return Ok(StatusCode::IllegalDataAddr),
            Err(..) => return Ok(StatusCode::DeviceFail),
        },
        None => return Ok(StatusCode::IllegalDataAddr),
    };

    /*if let Some(cb) = &inst.commit_coil_write {
        cb();
    }*/

    BigEndian::write_u16(&mut res.p[1..], addr);
    BigEndian::write_u16(&mut res.p[3..], value);
    res.size = 5;

    Ok(StatusCode::Ok)
}

pub fn write_multiple<'a>(
    inst: &'a Instance<'a>,
    buf: &[u8],
    res: &mut PDUBuf,
) -> Result<StatusCode, ()> {
    let coils = match &inst.coils {
        Some(c) => c,
        None => return Err(()),
    };

    // Check that request data is at least big enough for fields
    // fc', 'addr', 'nr of coils', 'byte count' and 'data'... (u8 + u16 + u16 + u8 + ...)
    if buf.len() < 7 {
        return Ok(StatusCode::IllegalDataValue);
    }

    if buf[0] != FunctionCode::WriteMultipleCoils as u8 {
        return Ok(StatusCode::DeviceFail);
    }

    let start_addr = BigEndian::read_u16(&buf[1..]);
    let quantity = BigEndian::read_u16(&buf[3..]);
    let byte_count = buf[5];

    if quantity == 0 || quantity > MAX_WRITE_QUANTITY {
        return Ok(StatusCode::IllegalDataValue);
    }

    if byte_count as u16 != ((quantity + 7) / 8) {
        return Ok(StatusCode::IllegalDataValue);
    }

    if buf.len() != (6 + byte_count).into() {
        return Ok(StatusCode::IllegalDataValue);
    }

    // Ensure all coils exist and can be written to before writing anything
    for i in 0..quantity {
        let addr = start_addr + i;
        match coil::find(addr, coils) {
            Some(c) => {
                if !c.write_allowed() {
                    return Ok(StatusCode::IllegalDataAddr);
                }
            }
            None => return Ok(StatusCode::IllegalDataAddr),
        }
    }

    // Write coils
    for i in 0..quantity {
        let addr = start_addr + i;
        let value = (buf[(6 + i / 8) as usize] & 1 << (i % 8)) != 0;
        match coil::find(addr, coils) {
            Some(c) => match c.write(value) {
                Ok(()) => (),
                Err(..) => return Ok(StatusCode::DeviceFail),
            },
            None => return Ok(StatusCode::DeviceFail),
        };
    }

    /*if let Some(ref mut cb) = inst.commit_coil_write {
        cb();
    }*/

    BigEndian::write_u16(&mut res.p[1..], start_addr);
    BigEndian::write_u16(&mut res.p[3..], quantity);
    res.size = 5;

    Ok(StatusCode::Ok)
}
