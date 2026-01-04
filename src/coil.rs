use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    ReadNotSuppported,
    WriteNotSuppported,
    ReadLocked,
    WriteLocked,
}

pub enum ReadMethod<'a> {
    Value(bool),
    Ref(&'a bool),
    Fn(fn() -> bool),
}

pub enum WriteMethod<'a> {
    Ref(&'a mut bool),
    Fn(&'a mut dyn FnMut(bool)),
}

pub struct Descriptor<'a> {
    pub address: u16,
    pub read: Option<ReadMethod<'a>>,
    pub write: Option<WriteMethod<'a>>,

    pub rlock: Option<fn() -> bool>,
    pub wlock: Option<fn() -> bool>,

    pub post_write: Option<fn() -> ()>,
}

impl<'a> Default for Descriptor<'a> {
    fn default() -> Self {
        Self {
            address: Default::default(),
            read: Default::default(),
            write: Default::default(),
            rlock: Default::default(),
            wlock: Default::default(),
            post_write: Default::default(),
        }
    }
}

impl<'a> PartialEq for Descriptor<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address
    }
}

impl<'a> Eq for Descriptor<'a> {}

impl<'a> PartialOrd for Descriptor<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.address.partial_cmp(&other.address)
    }
}

impl<'a> Ord for Descriptor<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.address.cmp(&other.address)
    }
}

impl<'a> Display for Descriptor<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:04X}", self.address)
    }
}

/// Finds a coild by it's address
///
/// This uses binary search, so all coils must be sorted in ascending order by address
pub fn find<'a>(address: u16, coils: &'a [Descriptor<'a>]) -> Option<&'a Descriptor<'a>> {
    match coils.binary_search_by(|coil| coil.address.cmp(&address)) {
        Ok(ix) => Some(&coils[ix]),
        Err(..) => None,
    }
}

impl<'a> Descriptor<'a> {
    pub fn read(&self) -> Result<bool, Error> {
        // Check read permissions
        match &self.rlock {
            Some(rlock) => {
                if rlock() {
                    return Err(Error::ReadLocked);
                }
            }
            None => (),
        };

        match &self.read {
            Some(method) => match method {
                ReadMethod::Value(v) => Ok(*v),
                ReadMethod::Ref(&r) => Ok(r),
                ReadMethod::Fn(f) => Ok(f()),
            },
            None => Err(Error::ReadNotSuppported),
        }
    }

    pub fn write(&mut self, value: bool) -> Result<(), Error> {
        // Check read permissions
        match &self.wlock {
            Some(wlock) => {
                if wlock() {
                    return Err(Error::WriteLocked);
                }
            }
            None => (),
        };

        match &mut self.write {
            Some(method) => {
                match method {
                    WriteMethod::Ref(_r) => {
                        //let r = *r;
                        //*r = value;
                        //*r = value;
                    }
                    WriteMethod::Fn(f) => f(value),
                };

                if let Some(cb) = self.post_write {
                    cb();
                }

                Ok(())
            }
            None => Err(Error::WriteNotSuppported),
        }
    }
}
