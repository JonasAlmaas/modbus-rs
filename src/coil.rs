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
    Fn(Box<dyn Fn() -> bool + 'a>),
}

pub enum WriteMethod<'a> {
    Ref(&'a mut bool),
    Fn(Box<dyn FnMut(bool) + 'a>),
}

pub struct Descriptor<'a> {
    pub address: u16,
    pub read: Option<ReadMethod<'a>>,
    pub write: Option<WriteMethod<'a>>,

    pub rlock: Option<Box<dyn Fn() -> bool + 'a>>,
    pub wlock: Option<Box<dyn Fn() -> bool + 'a>>,

    pub post_write: Option<Box<dyn FnMut() + 'a>>,
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

/// Finds a coild by it's address and get a mutable reference to it
///
/// This uses binary search, so all coils must be sorted in ascending order by address
pub fn find_mut<'a, 'b>(
    address: u16,
    coils: &'a mut [Descriptor<'b>],
) -> Option<&'a mut Descriptor<'b>> {
    match coils.binary_search_by(|coil| coil.address.cmp(&address)) {
        Ok(ix) => Some(&mut coils[ix]),
        Err(..) => None,
    }
}

impl<'a> Descriptor<'a> {
    pub fn read_allowed(&self) -> bool {
        match &self.rlock {
            Some(rlock) => rlock(),
            None => true,
        }
    }

    pub fn read(&self) -> Result<bool, Error> {
        if !self.read_allowed() {
            return Err(Error::ReadLocked);
        }

        match &self.read {
            Some(method) => match method {
                ReadMethod::Value(v) => Ok(*v),
                ReadMethod::Ref(&r) => Ok(r),
                ReadMethod::Fn(f) => Ok(f()),
            },
            None => Err(Error::ReadNotSuppported),
        }
    }

    pub fn write_allowed(&self) -> bool {
        match &self.wlock {
            Some(wlock) => !wlock(),
            None => true,
        }
    }

    pub fn write(&self, _value: bool) -> Result<(), Error> {
        if !self.write_allowed() {
            return Err(Error::WriteLocked);
        }

        match &self.write {
            Some(method) => {
                match method {
                    //WriteMethod::Ref(ref mut r) => **r = value,
                    //WriteMethod::Fn(f) => f(value),
                    WriteMethod::Ref(_r) => (),
                    WriteMethod::Fn(_f) => (),
                };

                /*if let Some(ref mut cb) = self.post_write {
                    cb();
                }*/

                Ok(())
            }
            None => Err(Error::WriteNotSuppported),
        }
    }
}
