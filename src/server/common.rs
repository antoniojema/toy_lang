use crate::{
    http::{HTTPMethod, HTTPRequest, HTTPHeader},
    common::typedef::TcpStream
};

pub type Handler = fn(&TcpStream, HTTPRequest);
pub type HandlerEmpty = fn(&TcpStream);

pub enum KeepAlive {
    Keep,
    Die
}

impl KeepAlive {
    pub fn from(header : &HTTPHeader) -> KeepAlive {
        match header.find_key("Keep-Alive", true) {
            Some(_) => KeepAlive::Keep,
            None => KeepAlive::Die
        }
    }
}

pub trait ServerImpl {
    fn register(&mut self, end_point : EndPoint);

    fn register_404(&mut self, handler : Handler);

    fn register_bad_req(&mut self, handler : HandlerEmpty);

    fn bind(&self, port: u16);
}

pub struct EndPoint {
    pub method : HTTPMethod,
    pub route : String,
    pub handler : Handler
}

impl EndPoint {
    pub fn matches(&self, request: &HTTPRequest) -> bool {
        return
            request.prelude.method == self.method &&
            request.prelude.route  == self.route;
    }
}

