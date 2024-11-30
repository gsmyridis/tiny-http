use std::convert::TryFrom;
use std::fmt;

use crate::error::InvalidVersion;

#[derive(Debug)]
enum Http {
    Http11,
    Http2,
}

#[derive(Debug)]
pub struct Version(Http);

impl Default for Version {
    #[inline]
    fn default() -> Version {
        Version::HTTP_11
    }
}

impl Version {
    /// `HTTP/1.1`
    pub const HTTP_11: Version = Version(Http::Http11);

    /// `HTTP/2.0
    pub const HTTP_2: Version = Version(Http::Http2);
}

impl<'a> TryFrom<&'a str> for Version {
    type Error = InvalidVersion;

    fn try_from(t: &'a str) -> Result<Self, Self::Error> {
        match t {
            "HTTP/1.1" => Ok(Version::HTTP_11),
            "HTTP/2.0" => Ok(Version::HTTP_2),
            _ => Err(InvalidVersion),
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::Http::*;

        f.write_str(match self.0 {
            Http11 => "HTTP/1.1",
            Http2 => "HTTP/2.0",
        })
    }
}
