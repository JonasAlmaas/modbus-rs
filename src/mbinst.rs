use crate::mbcoil;

pub struct SerialConfig {
    pub slave_addr: u8,
}

pub struct Instance<'a> {
    pub descriptors: Option<&'a [mbcoil::Descriptor<'a>]>,
    pub serial: Option<SerialConfig>,
}
