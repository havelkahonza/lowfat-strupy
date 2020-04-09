use std::sync::{RwLock, Arc};
use hyper::body::Bytes;

pub(crate) struct Payload {
    payload: Arc<RwLock<Bytes>>
}

impl Payload {
    pub fn new() -> Self {
        Payload{
            payload: Arc::new(RwLock::default())
        }
    }

    pub fn read(&self) -> Bytes {
        self.payload.read().expect("RwLock Error while reading").clone()
    }

    pub fn write(&self, bytes: Bytes) {
        *self.payload.write().expect("RwLock Error while writing") = bytes
    }
}