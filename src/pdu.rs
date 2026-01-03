use crate::def::{self, FunctionCode, StatusCode};
use crate::func;
use crate::Instance;

pub(crate) const PDU_DATA_SIZE_MAX: usize = 252;
pub const PDU_SIZE_MAX: usize = 1 + PDU_DATA_SIZE_MAX;

pub struct PDUBuf<'a> {
    pub p: &'a mut [u8; PDU_SIZE_MAX],
    pub size: usize,
}

fn handle_fn<'a>(inst: &'a Instance<'a>, buf: &[u8], res: &mut PDUBuf) -> StatusCode {
    match FunctionCode::try_from(buf[0]) {
        Ok(FunctionCode::ReadCoils) => match &inst.coils {
            Some(coils) => return func::coils::read_multiple(inst, coils, buf, res),
            None => (),
        },
        Ok(FunctionCode::ReadDiscreteInputs) => match &inst.disc_inputs {
            Some(disc_inputs) => return func::coils::read_multiple(inst, disc_inputs, buf, res),
            None => (),
        },
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

pub fn handle_req<'a>(inst: &'a Instance<'a>, buf: &[u8], res: &mut [u8; PDU_SIZE_MAX]) -> usize {
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::coil;

    #[test]
    fn pdu_too_little_data() {
        let inst: Instance = Default::default();

        let buf = [0x04];
        let mut res = [0; PDU_SIZE_MAX];
        let res_len = handle_req(&inst, &buf, &mut res);
        assert_eq!(res_len, 2);
        assert_eq!(res[0], 0x04 | 0x80); // Error response
        assert_eq!(res[1], 0x03); // Illegal data value
    }

    #[test]
    fn pdu_read_coil_works() {
        use coil::Descriptor as CoilDesc;

        let coil1 = false;

        let coils = &[
            CoilDesc {
                address: 0x00,
                read: Some(coil::ReadMethod::Value(true)),
                ..Default::default()
            },
            CoilDesc {
                address: 0x01,
                read: Some(coil::ReadMethod::Ref(&coil1)),
                ..Default::default()
            },
            CoilDesc {
                address: 0x02,
                read: Some(coil::ReadMethod::Fn(|| true)),
                ..Default::default()
            },
        ];
        let inst: Instance = Instance {
            coils: Some(coils),
            ..Default::default()
        };

        let buf = [
            0x01, // Fc: Read coils
            0x00, 0x00, // Start address
            0x00, 0x03, // Quantity
        ];
        let mut res = [0; PDU_SIZE_MAX];
        let res_len = handle_req(&inst, &buf, &mut res);
        assert_eq!(res_len, 3);
        assert_eq!(res[0], 0x01);
        assert_eq!(res[1], 0x01);
        assert_eq!(res[2], 0b0101);
    }

    #[test]
    fn pdu_write_single_coil_fn_works() {
        use coil::Descriptor as CoilDesc;

        let mut coil1 = false;
        let mut write_coil1 = |v: bool| coil1 = v;

        let coils = &[CoilDesc {
            address: 0x00,
            write: Some(coil::WriteMethod::Fn(&mut write_coil1)),
            ..Default::default()
        }];
        let _inst: Instance = Instance {
            coils: Some(coils),
            ..Default::default()
        };

        // TODO
        /*let buf = [
            0x01, // Fc: Read coils
            0x00, 0x00, // Start address
            0x00, 0x03, // Quantity
        ];
        let mut res = [0; PDU_SIZE_MAX];
        let res_len = handle_req(&inst, &buf, &mut res);*/
    }
}
