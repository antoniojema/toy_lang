use crate::common::{
    typedef::Bytes
};
use super::*;

#[derive(Debug)]
pub struct HTTPStatus {
    pub version : String,
    pub code : HTTPCode,
}

impl HTTPStatus {
    pub fn to_string(&self) -> String {
        format!("HTTP/{} {} {}", self.version, self.code as u16, self.code.to_string())
    }

    pub fn to_bytes(&self) -> Bytes {
        self.to_string().into_bytes()
    }
}
