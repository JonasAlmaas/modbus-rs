pub enum ReadMethod<'a> {
    Value(bool),
    Ref(&'a bool),
    Fn(fn() -> bool),
}

pub enum WriteMethod<'a> {
    Ref(&'a mut bool),
    Fn(fn() -> bool),
}

pub struct Descriptor<'a> {
    pub address: u16,
    pub read: Option<ReadMethod<'a>>,
    pub write: Option<WriteMethod<'a>>,

    pub rlock_cb: Option<fn() -> bool>,
    pub wlock_cb: Option<fn() -> bool>,

    pub post_write_cb: Option<fn() -> ()>,
}
