use crate::method::Method;
use crate::version::Version;
use crate::uri::Uri;

#[derive(Default)]
pub struct Parts {
    /// The request's method
    pub method: Method,

    /// The request's URI
    pub uri: Uri,

    /// The request's version
    pub version: Version,

}


impl Parts {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}
