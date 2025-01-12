use std::convert::TryFrom;
use std::fmt;
use std::hash::Hash;

use crate::error::InvalidUri;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Uri {
    pub inner: String,
}

impl Uri {
    /// Creates a new, canonicalized URI from a string reference.
    ///
    /// A canonical URI is one that starts with the prefix '/'.
    #[inline]
    pub fn new(uri: &str) -> Self {
        if !uri.starts_with("/") {
            Uri {
                inner: format!("/{}", uri),
            }
        } else {
            Uri {
                inner: uri.to_string(),
            }
        }
    }
}

impl PartialEq<str> for Uri {
    fn eq(&self, other: &str) -> bool {
        self.inner.as_str() == other
    }
}

impl Default for Uri {
    fn default() -> Self {
        Uri {
            inner: "/".to_string(),
        }
    }
}

impl fmt::Display for Uri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.inner)
    }
}

impl<'a> TryFrom<&'a str> for Uri {
    type Error = InvalidUri;

    fn try_from(t: &'a str) -> Result<Self, Self::Error> {
        Ok(Uri {
            inner: t.to_string(),
        })
    }
}
