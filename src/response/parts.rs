use crate::status::StatusCode;
use crate::version::Version;


pub struct Parts {
    /// The response's status
    pub status: StatusCode,

    /// The response's version
    pub version: Version,

}
