use crate::def::{self, FunctionCode, StatusCode};
use crate::func;
use crate::Instance;

const DATA_SIZE_MAX: usize = 252;
pub const SIZE_MAX: usize = 1 + DATA_SIZE_MAX;

pub struct PDUBuf<'a> {
    pub p: &'a mut [u8; SIZE_MAX],
    pub size: usize,
}

fn handle_fn<'a>(inst: &'a Instance<'a>, buf: &[u8], res: &mut PDUBuf) -> StatusCode {
    match FunctionCode::try_from(buf[0]) {
        Ok(FunctionCode::ReadCoils) => {
            if let Some(coils) = &inst.coils {
                return func::coils::read_multiple(inst, coils, buf, res);
            }
        }
        Ok(FunctionCode::ReadDiscreteInputs) => {
            if let Some(disc_inputs) = inst.disc_inputs {
                return func::coils::read_multiple(inst, disc_inputs, buf, res);
            }
        }
        Ok(FunctionCode::ReadHoldingRegs) => return func::regs::read_multiple(buf, res),
        Ok(FunctionCode::ReadInputRegs) => return func::regs::read_multiple(buf, res),
        Ok(FunctionCode::WriteSingleCoil) => (),
        Ok(FunctionCode::WriteSingleReg) => (),
        Ok(FunctionCode::ReadExceptionStatus) => (),
        Ok(FunctionCode::Diagnostics) => (),
        Ok(FunctionCode::CommEventCounter) => (),
        Ok(FunctionCode::CommEventLog) => (),
        Ok(FunctionCode::WriteMultipleCoils) => (),
        Ok(FunctionCode::WriteMultipleRegs) => (),
        Ok(FunctionCode::ReportSlaveId) => (),
        Ok(FunctionCode::ReadFileRecord) => (),
        Ok(FunctionCode::WriteFileRecord) => (),
        Ok(FunctionCode::MaskWriteReg) => (),
        Ok(FunctionCode::ReadWriteRegs) => (),
        Ok(FunctionCode::ReadFifoQueue) => (),
        Err(()) => (),
    };

    // If the library was not able to handle this request,
    // call the user defined hanlder function if present.
    if let Some(f) = inst.handle_fn {
        f(inst, buf, res)
    } else {
        StatusCode::IllegalFc
    }
}

pub fn handle_req<'a>(inst: &'a Instance<'a>, buf: &[u8], res: &mut [u8; SIZE_MAX]) -> usize {
    let fc = match buf.get(0) {
        Some(&b) => b,
        None => return 0,
    };

    let mut res = PDUBuf { p: res, size: 0 };

    res.p[0] = fc;
    res.size = 1;

    match handle_fn(inst, buf, &mut res) {
        StatusCode::Ok => (),
        status => {
            res.p[0] |= def::ERR_FLAG;
            res.p[1] = status as u8;
            res.size = 2;
        }
    };

    res.size
}
