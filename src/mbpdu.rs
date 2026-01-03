use crate::mbdef::{self, FunctionCode, StatusCode};
use crate::{Instance, mbfn_coils, mbfn_regs};

pub(crate) const PDU_DATA_SIZE_MAX: usize = 252;
pub const PDU_SIZE_MAX: usize = 1 + PDU_DATA_SIZE_MAX;

pub(crate) struct PDUBuf<'a> {
    pub p: &'a mut [u8; PDU_SIZE_MAX],
    pub size: usize,
}

fn handle_fn(_inst: &Instance, buf: &[u8], res: &mut PDUBuf) -> StatusCode {
    match FunctionCode::try_from(buf[0]) {
        Ok(FunctionCode::ReadCoils) => mbfn_coils::read_multiple(buf, res),
        Ok(FunctionCode::ReadDiscreteInputs) => mbfn_coils::read_multiple(buf, res),
        Ok(FunctionCode::ReadHoldingRegs) => mbfn_regs::read_multiple(buf, res),
        Ok(FunctionCode::ReadInputRegs) => mbfn_regs::read_multiple(buf, res),
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

pub fn handle_req(inst: &Instance, buf: &[u8], res: &mut [u8; PDU_SIZE_MAX]) -> usize {
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
            res.p[0] |= mbdef::ERR_FLAG;
            res.p[1] = status as u8;
            res.size = 2;
        }
    };

    res.size
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::SerialConfig;

    #[test]
    fn mbpdu_too_little_data() {
        let inst = Instance {
            descriptors: None,
            serial: Some(SerialConfig { slave_addr: 1 }),
        };

        let buf = [0x04];
        let mut res = [0; PDU_SIZE_MAX];
        let res_len = handle_req(&inst, &buf, &mut res);
        assert_eq!(res_len, 2);
        assert_eq!(res[0], 0x04 | 0x80); // Error response
        assert_eq!(res[1], 0x03); // Illegal data value
    }
}
