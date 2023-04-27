use super::*;

#[derive(Debug)]
pub struct HTTPResponse {
    pub status : HTTPStatus,
    pub header : HTTPHeader,
    pub body : HTTPBody
}
