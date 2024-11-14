use std::error::Error;
use std::fmt;
use std::convert::TryFrom;


#[derive(Debug, PartialEq)]
pub struct Uri {
    pub inner: String,
}


impl PartialEq<str> for Uri {
    fn eq(&self, other: &str) -> bool {
        self.inner.as_str() == other
    }
}


impl Default for Uri {
    fn default() -> Self {
        Uri { inner: "/".to_string() }
    }
}

#[derive(Debug)]
pub struct InvalidUri;


impl fmt::Display for InvalidUri {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Invalid Uri")
    }
}


impl Error for InvalidUri {}


impl<'a> TryFrom<&'a str> for Uri {
    type Error = InvalidUri;

    fn try_from(t: &'a str) -> Result<Self, Self::Error> {
        Ok(Uri {inner: t.to_string() })
    }
}
