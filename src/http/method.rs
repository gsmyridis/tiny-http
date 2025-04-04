use std::convert::TryFrom;
use std::fmt::{self, Display, Formatter};
use std::hash::Hash;

use self::Inner::*;
use crate::error::InvalidMethod;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Inner {
    Get,
    Post,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Method(Inner);

impl Default for Method {
    fn default() -> Self {
        Method::GET
    }
}

impl Method {
    // GET
    pub const GET: Method = Method(Get);

    // POST
    pub const POST: Method = Method(Post);

    pub fn as_str(&self) -> &str {
        match self.0 {
            Get => "GET",
            Post => "POST",
        }
    }
}

impl<'a> TryFrom<&'a str> for Method {
    type Error = InvalidMethod;

    #[inline]
    fn try_from(t: &'a str) -> Result<Self, Self::Error> {
        match t {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            _ => Err(InvalidMethod),
        }
    }
}

impl Display for Method {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
