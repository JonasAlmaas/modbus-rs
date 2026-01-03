use crate::{def::StatusCode, pdu::PDUBuf};

pub mod adu;
pub mod coil;
mod crc;
pub mod def;
mod func;
pub mod pdu;

pub struct SerialConfig {
    pub slave_addr: u8,
}

pub struct Instance<'a> {
    pub disc_inputs: Option<&'a [coil::Descriptor<'a>]>,
    pub coils: Option<&'a [coil::Descriptor<'a>]>,

    pub handle_fn: Option<fn(inst: &Instance, buf: &[u8], res: &mut PDUBuf) -> StatusCode>,

    pub serial: Option<SerialConfig>,
}

impl<'a> Default for Instance<'a> {
    fn default() -> Self {
        Self {
            disc_inputs: Default::default(),
            coils: Default::default(),
            handle_fn: Default::default(),
            serial: Default::default(),
        }
    }
}

impl<'a> Instance<'a> {
    pub fn init(&mut self) {
        // TODO: Initialize internla state
    }
}
