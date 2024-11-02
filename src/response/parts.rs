use crate::status::StatusCode;
use crate::version::Version;


#[derive(Default)]
pub struct Parts {
    /// The response's status
    pub status: StatusCode,

    /// The response's version
    pub version: Version,

}


impl Parts {
    #[inline]
    pub fn new() -> Self {
        Parts::default()
    }
}
