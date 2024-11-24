use crate::status::StatusCode;
use crate::version::Version;
use crate::header::HeaderMap;


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
