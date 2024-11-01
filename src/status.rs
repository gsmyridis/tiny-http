use std::num::NonZeroU16;
use std::convert::TryFrom;


/// An HTTP status code.
pub struct StatusCode(NonZeroU16);


/// Error for invalid status code.
#[derive(Debug)]
pub struct InvalidStatusCode;


impl TryFrom<u16> for StatusCode {
    type Error = InvalidStatusCode;

    fn try_from(num: u16) -> Result<StatusCode, Self::Error> {
        NonZeroU16::new(num).map(StatusCode).ok_or_else(|| InvalidStatusCode)
    }
}


impl<'a> TryFrom<&'a str> for StatusCode {
    type Error = InvalidStatusCode;

    fn try_from(s: &'a str) -> Result<StatusCode, Self::Error> {
        let num = s.parse::<u16>().map_err(|_| InvalidStatusCode)?;
        StatusCode::try_from(num)
    }
}

