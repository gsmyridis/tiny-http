pub mod pool;
pub mod worker;

use std::net::{TcpListener, ToSocketAddrs};
use std::io;


pub struct HttpServer {
    pub listener: TcpListener,
}


impl HttpServer {
    
    pub fn bind(addr: impl ToSocketAddrs) -> io::Result<Self> {
        let listener = TcpListener::bind(addr)?;
        Ok(Self { listener })
    }
}


