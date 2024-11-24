use bytes::Bytes;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct HeaderValue {
    inner: Bytes,
}

impl HeaderValue {
    /// Consumes the `HeaderValue` struct and return the value in `Bytes`.
    pub fn as_bytes(&self) -> &Bytes {
        &self.inner
    }
}

impl<const N: usize> From<&[u8; N]> for HeaderValue {
    fn from(slice: &[u8; N]) -> Self {
        Self {
            inner: Bytes::copy_from_slice(slice),
        }
    }
}

impl From<&[u8]> for HeaderValue {
    fn from(slice: &[u8]) -> Self {
        Self {
            inner: Bytes::copy_from_slice(slice),
        }
    }
}

impl From<&str> for HeaderValue {
    fn from(slice: &str) -> Self {
        Self {
            inner: Bytes::copy_from_slice(slice.as_bytes()),
        }
    }
}

impl fmt::Display for HeaderValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(self.inner.as_ref()))
    }
}
