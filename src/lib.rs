pub mod mbadu;
pub mod mbcoil;
mod mbcrc;
pub mod mbdef;
mod mbfn_coils;
mod mbfn_regs;
pub mod mbpdu;

pub struct SerialConfig {
    pub slave_addr: u8,
}

pub struct Instance<'a> {
    pub descriptors: Option<&'a [mbcoil::Descriptor<'a>]>,
    pub serial: Option<SerialConfig>,
}

impl<'a> Instance<'a> {
    pub fn init(&mut self) {
        // TODO: Initialize internla state
    }
}
