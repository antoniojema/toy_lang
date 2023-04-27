use super::common::*;
use crate::{
    common::typedef::*,
    http::*
};

pub struct Server {
    end_points : Vec<EndPoint>,
    handler_404 : Option<Handler>,
    handler_badreq : Option<HandlerEmpty>
}

impl Server {
    pub fn new() -> Server {
        Server{
            end_points : vec![],
            handler_404 : None,
            handler_badreq : None,
        }
    }

    fn find_end_point<'a>(&'a self, request : &HTTPRequest) -> Option<&'a EndPoint> {
        for p in self.end_points.iter() {
            if p.matches(request) {return Some(p);}
        }
        None
    }

    fn handle_404(&self, stream : &TcpStream, request : HTTPRequest) {
        match self.handler_404 {
            Some(v) => {v(stream, request)},
            None => {
                //TODO
            }
        }
    }

    fn handle_badreq(&self, stream : &TcpStream) {
        match self.handler_badreq {
            Some(v) => {v(stream)},
            None => {
                //TODO
            }
        }
    }

    fn handle_connection(&self, stream : TcpStream) {
        let mut reader = BufReader::new(stream.try_clone().unwrap());
        loop {
            let request = match HTTPRequest::from(&mut reader){
                Ok(Some(v)) => v,
                _ => {
                    self.handle_badreq(&stream);
                    break;
                }
            };

            let keep_alive = KeepAlive::from(&request.header);

            let end_point = match self.find_end_point(&request) {
                Some(p) => p,
                None => {
                    self.handle_404(&stream, request);
                    match keep_alive {
                        KeepAlive::Keep => continue,
                        KeepAlive::Die => break
                    }
                }
            };

            (end_point.handler)(&stream, request);
        }
    }
}

impl ServerImpl for Server {
    fn register(&mut self, end_point : EndPoint) {
        self.end_points.push(end_point);
    }

    fn register_404(&mut self, handler : Handler) {
        self.handler_404 = Some(handler);
    }

    fn register_bad_req(&mut self, handler : HandlerEmpty) {
        self.handler_badreq = Some(handler);
    }
    
    fn bind(&self, port: u16) {
        let listener = TcpListener::bind(format!("127.0.0.1:{port}")).unwrap();
    
        thread::scope(|scope| {
            for stream in listener.incoming() {
                let stream = stream.unwrap();
    
                scope.spawn(||
                    self.handle_connection(stream)
                );
            }
        });
    }
}