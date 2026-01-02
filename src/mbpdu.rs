use std::{u8, usize};

use crate::mbdef::{self, FunctionCode, StatusCode};

pub const PDU_DATA_SIZE_MAX: usize = 252;
pub const PDU_SIZE_MAX: usize = 1 + PDU_DATA_SIZE_MAX;

struct PDUBuf<'a> {
    p: &'a mut [u8],
    size: usize,
}

fn read_coils(buf: &[u8], res: &mut PDUBuf) -> StatusCode {
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

fn read_regs(buf: &[u8], res: &mut PDUBuf) -> StatusCode {
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

fn handle_fn(buf: &[u8], res: &mut PDUBuf) -> StatusCode {
    match FunctionCode::try_from(buf[0]) {
        Ok(FunctionCode::ReadCoils) => read_coils(buf, res),
        Ok(FunctionCode::ReadDiscreteInputs) => StatusCode::IllegalFc,
        Ok(FunctionCode::ReadHoldingRegs) => read_regs(buf, res),
        Ok(FunctionCode::ReadInputRegs) => read_regs(buf, res),
        Ok(FunctionCode::WriteSingleCoil) => StatusCode::IllegalFc,
        Ok(FunctionCode::WriteSingleReg) => StatusCode::IllegalFc,
        Ok(FunctionCode::ReadExceptionStatus) => StatusCode::IllegalFc,
        Ok(FunctionCode::Diagnostics) => StatusCode::IllegalFc,
        Ok(FunctionCode::CommEventCounter) => StatusCode::IllegalFc,
        Ok(FunctionCode::CommEventLog) => StatusCode::IllegalFc,
        Ok(FunctionCode::WriteMultipleCoils) => StatusCode::IllegalFc,
        Ok(FunctionCode::WriteMultipleRegs) => StatusCode::IllegalFc,
        Ok(FunctionCode::ReportSlaveId) => StatusCode::IllegalFc,
        Ok(FunctionCode::ReadFileRecord) => StatusCode::IllegalFc,
        Ok(FunctionCode::WriteFileRecord) => StatusCode::IllegalFc,
        Ok(FunctionCode::MaskWriteReg) => StatusCode::IllegalFc,
        Ok(FunctionCode::ReadWriteRegs) => StatusCode::IllegalFc,
        Ok(FunctionCode::ReadFifoQueue) => StatusCode::IllegalFc,
        Err(()) => StatusCode::IllegalFc,
    }
}

pub fn handle_req(buf: &[u8], res: &mut [u8]) -> usize {
    let fc = match buf.get(0) {
        Some(&b) => b,
        None => return 0,
    };

    let mut res = PDUBuf { p: res, size: 0 };

    res.p[0] = fc;
    res.size = 1;

    match handle_fn(buf, &mut res) {
        StatusCode::Ok => (),
        status => {
            res.p[0] |= mbdef::ERR_FLAG;
            res.p[1] = status as u8;
            res.size = 2;
        }
    };

    res.size
}
