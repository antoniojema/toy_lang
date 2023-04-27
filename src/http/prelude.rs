use crate::common::{
    macros::*,
    typedef::*
};
use super::*;

#[derive(Debug)]
pub struct HTTPPrelude {
    pub method : HTTPMethod,
    pub route : String,
    pub version : String,
}

impl HTTPPrelude {
    pub fn has_error(line : &str) -> Option<String> {
        let vals : Vec<&str> = line.split(' ').collect();
        
        if vals.len() != 3 { return Some(String::from("Error: Invalid HTTP prelude")); }

        if vals[1].len() == 0 || &vals[1][0..1] != "/" { return Some(String::from("Error: Could not read HTTP route")); }

        if vals[2].len() <= 6 || &vals[2][..5] != "HTTP/" { return Some(String::from("Error: Could not read HTTP version."));}

        match vals[0] {
            "GET"|"PUT"|"POST" => (),
            other => {return Some(format!("Error: Invalid HTTP method: {}", other))}
        }

        None
    }

    pub fn from(reader : &mut BufReader<TcpStream>) -> Result<Option<HTTPPrelude>, String> {
        let line = unwrap_result_or_return!(
            unwrap_option_or_return!(
                reader.lines().next(),
                Ok(None)
            ),
            Err(String::from("Error: Could not read stream line."))
        );
        
        match HTTPPrelude::has_error(&line) {
            Some(err) => {return Err(err);}
            None => ()
        }

        let vals : Vec<&str> = line.split(' ').collect();

        let method = match vals[0] {
            "GET" => HTTPMethod::GET,
            "PUT" => HTTPMethod::PUT,
            "POST" => HTTPMethod::POST,
            _ => {return Err(String::from("Unreachable"))}
        };

        let route = String::from((&vals[1][..]).trim());

        let version = String::from((&vals[2][5..]).trim());

        Ok(Some(HTTPPrelude {method, route, version}))
    }
}