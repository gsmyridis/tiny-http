use crate::http::header::HeaderMap;
use crate::http::status::StatusCode;
use crate::http::version::Version;

#[derive(Default)]
pub struct Parts {
    /// The response's status
    pub status: StatusCode,

    /// The response's version
    pub version: Version,

    /// Header-map
    pub headers: HeaderMap,
}

impl Parts {
    #[inline]
    pub fn new() -> Self {
        Parts::default()
    }
}
