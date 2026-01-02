use std::convert::TryFrom;

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum StatusCode {
    Ok = 0x00,
    IllegalFc = 0x01, // Function code received in the query is not recognized or allowed.
    _IllegalDataAddr = 0x02, // Data address of some or all the required entities are not allowed or do not exist.
    IllegalDataValue = 0x03, // Illegal data value. Value is not accepted.
    DeviceFail = 0x04, // Unrecoverable error occurred while slave was attempting to perform requested action.
    /* Slave has accepted request and is processing it, but a long duration of time is required.
    This response is returned to prevent a timeout error from occurring on the master.
    The master can next issue a Poll Program Complete message to determine whether processing is completed */
    _Acknowlage = 0x05,
    _Busy = 0x06, // Slave is engaged in processing a long-duration command; client should retry later
    /* Slave cannot perform the programming functions;
    master should request diagnostic or error information from slave */
    _NegaticeAcknowlage = 0x07,
    _MemoryParityError = 0x08, // Slave detected a parity error in memory; master can retry the request
}

/**
 * Modbus error flag
 * Added onto the functino code for error responses
 */
pub const ERR_FLAG: u8 = 0x8;

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum FunctionCode {
    ReadCoils = 0x01,
    ReadDiscreteInputs = 0x02,
    ReadHoldingRegs = 0x03,
    ReadInputRegs = 0x04,
    WriteSingleCoil = 0x05,
    WriteSingleReg = 0x06,
    ReadExceptionStatus = 0x07, // Serial only
    Diagnostics = 0x08,         // Serial only
    CommEventCounter = 0x0B,    // Serial only
    CommEventLog = 0x0C,        // Serial only
    WriteMultipleCoils = 0x0F,
    WriteMultipleRegs = 0x10,
    ReportSlaveId = 0x11, // Serial only
    ReadFileRecord = 0x14,
    WriteFileRecord = 0x15,
    MaskWriteReg = 0x16,
    ReadWriteRegs = 0x17,
    ReadFifoQueue = 0x18,
}

impl TryFrom<u8> for FunctionCode {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(FunctionCode::ReadCoils),
            0x02 => Ok(FunctionCode::ReadDiscreteInputs),
            0x03 => Ok(FunctionCode::ReadHoldingRegs),
            0x04 => Ok(FunctionCode::ReadInputRegs),
            0x05 => Ok(FunctionCode::WriteSingleCoil),
            0x06 => Ok(FunctionCode::WriteSingleReg),
            0x07 => Ok(FunctionCode::ReadExceptionStatus),
            0x08 => Ok(FunctionCode::Diagnostics),
            0x0B => Ok(FunctionCode::CommEventCounter),
            0x0C => Ok(FunctionCode::CommEventLog),
            0x0F => Ok(FunctionCode::WriteMultipleCoils),
            0x10 => Ok(FunctionCode::WriteMultipleRegs),
            0x11 => Ok(FunctionCode::ReportSlaveId),
            0x14 => Ok(FunctionCode::ReadFileRecord),
            0x15 => Ok(FunctionCode::WriteFileRecord),
            0x16 => Ok(FunctionCode::MaskWriteReg),
            0x17 => Ok(FunctionCode::ReadWriteRegs),
            0x18 => Ok(FunctionCode::ReadFifoQueue),
            _ => Err(()),
        }
    }
}
