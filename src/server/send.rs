use crate::{
    common::{
        typedef::*,
        macros::*
    },
    http::*
};
use std::{
    io::Write,
    fs
};

fn remove_trailing_slash(s : &str) -> String {
    match s.chars().last() {
        Some('/') => {
            let mut chars = s.chars();
            chars.next_back();
            String::from(chars.as_str())
        },
        _ => String::from(s)
    }
}

fn add_leading_slash(s : &str) -> String {
    match s.chars().next() {
        Some('/') => String::from(s),
        _ => format!("/{s}")
    }
}

fn get_relative_path(route : &str, root : &str) -> String {
    let route = remove_trailing_slash(add_leading_slash(route).as_str());
    let root = remove_trailing_slash(root);

    format!("{root}{route}")
}

pub fn send_http(mut stream : TcpStream, response : &HTTPResponse) -> Result<(), ()> {
    unwrap_result_or_return!(stream.write_all(&response.status.to_bytes()[..]), Err(()));
    unwrap_result_or_return!(stream.write_all("\r\n".as_bytes()), Err(()));
    unwrap_result_or_return!(stream.write_all(&response.header.to_bytes()[..]), Err(()));
    unwrap_result_or_return!(stream.write_all("\r\n\r\n".as_bytes()), Err(()));
    if response.body.contents.len() > 0 {
        unwrap_result_or_return!(stream.write_all(&response.body.contents[..]), Err(()));
    };
    Ok(())
}

pub fn send_contents(stream : TcpStream, contents : Vec<u8>) -> Result<(), ()> {
    let length = contents.len();
    Ok(send_http(
        stream.try_clone().unwrap(),
        &HTTPResponse{
            status: HTTPStatus {version: format!("1.1"), code: HTTPCode::Ok },
            header: HTTPHeader{entries: vec![
                HTTPHeaderEntry{key: format!("Content-Length"), value: format!("{length}")}
            ]},
            body: HTTPBody::from(contents)
        }
    ))?
}

pub fn send_file(stream : TcpStream, route : &str, root : &str) -> Result<(), String> {
    let relative_path = get_relative_path(route, root);
    
    let relative_path = match fs::metadata(relative_path.as_str()) {
        Err(_) => return Err(String::from("Route does not exist.")),
        Ok(metadata) => format!(
            "{relative_path}{}",
            if metadata.is_dir() {"/index.html"} else {""}
        )
    };

    let contents = match fs::read(relative_path) {
        Ok(v) => v,
        Err(_) => return Err(String::from("File not found"))
    };

    unwrap_result_or_return!(
        send_contents(stream, contents),
        Err(String::from("Could not send http response."))
    );

    Ok(())
}

pub fn send_empty_ok(stream : &TcpStream) {
    send_http(
        stream.try_clone().unwrap(),
        &HTTPResponse {
            status: HTTPStatus {version: format!("1.1"), code: HTTPCode::Ok},
            header: HTTPHeader::empty(),
            body: HTTPBody::empty()
        }
    ).unwrap_or(());
}
