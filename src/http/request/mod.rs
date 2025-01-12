pub mod build;
pub mod parts;

use std::fmt;
use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;

use crate::error::*;
use crate::http::*;
use build::Builder;
use parts::Parts;

use bytes::Bytes;

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
    pub fn from_stream(stream: &mut TcpStream) -> Result<Request<Bytes>> {
        let mut bufreader = BufReader::new(stream);

        // Parse the request-line
        let mut request_line = String::new();
        bufreader
            .read_line(&mut request_line)
            .expect("Failed to read first line.");
        let mut request_line = request_line.trim().split(' ');
        let mut request = Request::builder()
            .with_method(
                request_line
                    .next()
                    .ok_or_else(|| Error::from(InvalidMethod))?,
            )
            .with_uri(request_line.next().ok_or_else(|| Error::from(InvalidUri))?)
            .with_version(
                request_line
                    .next()
                    .ok_or_else(|| Error::from(InvalidVersion))?,
            );

        // Parse the header lines
        let mut header_line = String::new();
        let mut content_len = "0".to_string();
        while bufreader.read_line(&mut header_line).is_ok() {
            if header_line == "\r\n" {
                break;
            }
            if let Some((name, val)) = header_line.trim().split_once(": ") {
                if name == "Content-Length" {
                    content_len = val.to_string();
                }
                request = request.with_header(name.as_bytes(), val.as_bytes());
                header_line.clear();
            } else {
                panic!("Invalid header line {header_line}");
            }
        }

        // Parse the request's body
        let len = content_len
            .parse::<usize>()
            .map_err(|_| Error::from(InvalidHeaderName))?;
        let mut body = vec![0_u8; len];
        let _ = bufreader.read_exact(&mut body);
        request.with_body(Bytes::copy_from_slice(&body))
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

    /// Returns a reference to the Request's `HeaderMap`.
    #[inline]
    pub fn headers(&self) -> &HeaderMap {
        &self.head.headers
    }

    /// Returns the header-value for the specified header-name.
    pub fn get_header<H>(&self, name: H) -> Option<&HeaderValue>
    where
        HeaderName: TryFrom<H>,
    {
        let header_name = TryFrom::try_from(name).ok()?;
        self.head.headers.get(&header_name)
    }

    /// Returns a reference to the body of the `Request`.
    #[inline]
    pub fn body(&self) -> &T {
        &self.body
    }
}

impl<T> fmt::Display for Request<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}\r\n", self.method(), self.uri(), self.version())?;
        for (name, val) in self.headers() {
            write!(f, "{}: {}\r\n", name, val)?;
        }
        write!(f, "\r\n{}", self.body)
    }
}
