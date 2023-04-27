use crate::common::typedef::Bytes;

#[derive(Debug)]
pub struct HTTPBody {
    pub contents: Bytes
}

impl HTTPBody {
    pub fn empty() -> HTTPBody {
        HTTPBody { contents: Bytes::new() }
    }

    pub fn from(vec: Vec<u8>) -> HTTPBody {
        HTTPBody { contents: vec }
    }
}
