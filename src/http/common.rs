#[derive(Debug, std::cmp::PartialEq)]
pub enum HTTPMethod{
    GET, POST, PUT
}

#[derive(Debug, Clone, Copy)]
#[repr(u16)]
pub enum HTTPCode{
    Ok = 200,
    BadReq = 400,
    NotFound = 404
}

impl HTTPCode {
    pub fn to_string(&self) -> String {
        String::from(match self {
            Self::Ok => "OK",
            Self::BadReq => "Invalid",
            Self::NotFound => "OK"
        })
    }
}
