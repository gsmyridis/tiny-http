use std::fmt;
use bytes::Bytes;


/// 
#[derive(Debug)]
pub struct HeaderValue {
    inner: Bytes
}

impl<const N: usize> From<&[u8; N]> for HeaderValue {
    fn from(slice: &[u8; N]) -> Self {
        Self{inner: Bytes::copy_from_slice(slice) }
    }
}

impl From<&[u8]> for HeaderValue {
    fn from(slice: &[u8]) -> Self {
        Self{inner: Bytes::copy_from_slice(slice) }
    }
}

impl fmt::Display for HeaderValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(self.inner.as_ref()))
    }
}
