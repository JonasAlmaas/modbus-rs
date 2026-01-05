pub mod adu;
pub mod adu_tcp;
pub mod coil;
pub mod crc;
mod def;
mod func;
pub mod pdu;

pub use crate::def::{FunctionCode, StatusCode};
use crate::pdu::PDUBuf;

pub struct SerialConfig {
    pub slave_addr: u8,
}

pub struct Instance<'a> {
    pub disc_inputs: Option<&'a [coil::Descriptor<'a>]>,
    pub coils: Option<&'a [coil::Descriptor<'a>]>,

    pub handle_fn: Option<Box<dyn FnMut(&Instance, &[u8], &mut PDUBuf) -> StatusCode + 'a>>,

    pub commit_coil_write: Option<Box<dyn FnMut() + 'a>>,

    pub serial: Option<SerialConfig>,
}

impl<'a> Default for Instance<'a> {
    fn default() -> Self {
        Self {
            disc_inputs: Default::default(),
            coils: Default::default(),
            handle_fn: Default::default(),
            commit_coil_write: Default::default(),
            serial: Default::default(),
        }
    }
}

impl<'a> Instance<'a> {
    pub fn init(&mut self) {
        // TODO: Initialize internla state
    }
}

#[macro_export]
macro_rules! asc {
    ($($item:expr),+ $(,)?) => {{
        let arr = [$($item),+];

        let mut i = 0;
        while i + 1 < arr.len() {
            if arr[i] > arr[i + 1] {
                panic!("asc! requires items to be sorted in ascending order");
            }
            i += 1;
        }

        arr
    }};
}
