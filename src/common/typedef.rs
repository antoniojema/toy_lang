pub type Bytes = Vec<u8>;

pub use std::{
    net::{
        TcpListener,
        TcpStream
    },
    io::{
        BufReader,
        BufRead,
        Read
    },
    thread
};
