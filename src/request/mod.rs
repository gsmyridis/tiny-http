pub mod parts;
pub mod build;

use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;

use crate::method::{Method, InvalidMethod};
use crate::version::{Version, InvalidVersion};
use crate::uri::{Uri, InvalidUri};
use crate::parts::Parts;
use crate::request::build::Builder;
use crate::error::{Error, Result};


pub struct Request<T> {
    head: Parts,
    body: T,
}



impl Request<()> {

    /// Returns a `Builder` that constructs a `Request`.
    #[inline]
    pub fn builder() -> Builder {
        Builder::new()
    }

    pub fn from_stream(stream: &mut TcpStream) -> Result<Request<String>> {
         
        let bufreader = BufReader::new(stream);
        let mut request_lines = bufreader.lines();

        let request_header = request_lines.next()
            .expect("Failed to read next line from request reader.")
            .expect("There is no header line.");
        let mut header_split = request_header.split(' ');
  
        let request = Request::builder()
           .with_method(header_split.next().ok_or_else(|| Error::from(InvalidMethod))?)
           .with_uri(header_split.next().ok_or_else(|| Error::from(InvalidUri))?)
           .with_version(header_split.next().ok_or_else(|| Error::from(InvalidVersion))?)
           .with_body("".to_string());
       
        request
    }
    
}


impl<T> Request<T> {

    /// Returns a reference to the HTTP method of the `Request`.
    #[inline]
    pub fn method(&self) -> &Method {
        &self.head.method
    }

    /// Returns a reference to the URI of the `Request`.
    #[inline]
    pub fn uri(&self) -> &Uri {
        &self.head.uri
    }

    /// Returns the HTTP version of the `Request`.
    #[inline]
    pub fn version(&self) -> &Version {
        &self.head.version
    }

    /// Returns a reference to the body of the `Request`.
    #[inline]
    pub fn body(&self) -> &T {
        &self.body
    }
}
