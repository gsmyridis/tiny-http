pub mod build;
pub mod parts;

use std::io::{BufRead, BufReader};
use std::net::TcpStream;

use parts::Parts;
use build::Builder;
use crate::method::{Method, InvalidMethod};
use crate::version::{Version, InvalidVersion};
use crate::uri::{Uri, InvalidUri};
use crate::error::{Error, Result};
use crate::header::value::HeaderValue;
use crate::header::name::HeaderName;


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

    /// Creates a new `Request` from a TCP Stream.
    pub fn from_stream(stream: &mut TcpStream) -> Result<Request<String>> {
        let mut bufreader = BufReader::new(stream);

        // Parse the reqeust's request line
        let mut request_line = String::new();
        bufreader.read_line(&mut request_line).expect("Failed to read first line.");
        let mut request_line = request_line.trim().split(' ');
        let mut request = Request::builder()
           .with_method(request_line.next().ok_or_else(|| Error::from(InvalidMethod))?)
           .with_uri(request_line.next().ok_or_else(|| Error::from(InvalidUri))?)
           .with_version(request_line.next().ok_or_else(|| Error::from(InvalidVersion))?);

        // Parse the request's header lines
        let mut header_line = String::new();
        while let Ok(_) = bufreader.read_line(&mut header_line) {
            if header_line == "\r\n" {
                break;
            }
            if let Some((name, val)) = header_line.trim().split_once(": ") {
                request = request.with_header(name.as_bytes(), val.as_bytes());
                header_line.clear();
            } else {
                panic!("Invalid header line");
            }
        }

        // Parse the request's body
        // let body = lines.map(|line| line.expect("")).collect::<Vec<_>>().join(" ");
        request.with_body("".to_string())
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

    /// Returns the header-value for the specified header-name.
    #[inline]
    pub fn get_header(&self, name: &HeaderName) -> Option<&HeaderValue> {
       self.head.headers.get(name)
    }

    /// Returns a reference to the body of the `Request`.
    #[inline]
    pub fn body(&self) -> &T {
        &self.body
    }
}
