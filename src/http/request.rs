use crate::common::{
    macros::*,
    typedef::*,
    traits::*
};
use super::*;

#[derive(Debug)]
pub struct HTTPRequest {
    pub prelude : HTTPPrelude,
    pub header : HTTPHeader,
    pub body : HTTPBody
}

impl HTTPRequest {
    fn read_body_from_bytes(reader : &mut BufReader<TcpStream>, n_bytes : usize) -> Result<HTTPBody, String> {
        let mut buff = Bytes::with_size(n_bytes, &0);
        unwrap_result_or_return!(
            reader.read_exact(&mut buff[..]),
            Err(String::from("Error: Could not read HTTP body."))
        );

        Ok(HTTPBody{contents: buff})
    }

    fn read_body_from_header(reader : &mut BufReader<TcpStream>, header : &HTTPHeader) -> Result<HTTPBody, String> {
        let key_index = unwrap_option_or_return!(
            header.find_key("Content-Length", true),
            Ok(HTTPBody::empty())
        );

        let n_bytes = unwrap_result_or_return!(
            str::parse::<usize>(&header.entries[key_index].value[..]),
            Err(String::from("Error: Could not parse Content-Length."))
        );

        HTTPRequest::read_body_from_bytes(reader, n_bytes)
    }

    pub fn from(reader : &mut BufReader<TcpStream>) -> Result<Option<HTTPRequest>, String> {
        let prelude = match unwrap_or_return!(HTTPPrelude::from(reader)) {
            Some(val) => val,
            None => {return Ok(None);}
        };

        let header = unwrap_or_return!(HTTPHeader::from(reader));

        let body = unwrap_or_return!(HTTPRequest::read_body_from_header(reader, &header));

        Ok(Some(HTTPRequest {
            prelude,
            header,
            body
        }))
    }
}
