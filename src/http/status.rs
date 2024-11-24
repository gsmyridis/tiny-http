use std::convert::TryFrom;
use std::fmt;
use std::num::NonZeroU16;

/// An HTTP status code.
#[derive(Debug)]
pub struct StatusCode(NonZeroU16);

/// Error for invalid status code.
#[derive(Debug)]
pub struct InvalidStatusCode;

impl Default for StatusCode {
    fn default() -> Self {
        StatusCode::OK
    }
}

impl StatusCode {
    /// Returns the code of the status.
    pub fn code(&self) -> NonZeroU16 {
        self.0
    }

    /// Returns the canonical reason that corresponds to the status code.
    pub fn msg(&self) -> Option<&'static str> {
        canonical_reason(self.0.into())
    }
}

impl TryFrom<u16> for StatusCode {
    type Error = InvalidStatusCode;

    fn try_from(num: u16) -> Result<StatusCode, Self::Error> {
        NonZeroU16::new(num)
            .map(StatusCode)
            .ok_or(InvalidStatusCode)
    }
}

impl<'a> TryFrom<&'a str> for StatusCode {
    type Error = InvalidStatusCode;

    fn try_from(s: &'a str) -> Result<StatusCode, Self::Error> {
        let num = s.parse::<u16>().map_err(|_| InvalidStatusCode)?;
        StatusCode::try_from(num)
    }
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let reason = canonical_reason(self.0.into()).expect("Failed to get canonical reason");
        write!(f, "{} {}", self.0, reason)
    }
}

macro_rules! status_codes {
    (
        $(
            $(#[$docs:meta])*
            ($num:expr, $konst:ident, $phrase:expr);
        )+
    ) => {
        impl StatusCode {
            $(
                pub const $konst: StatusCode = StatusCode( unsafe { NonZeroU16::new_unchecked($num) });
            )+
        }

        pub fn canonical_reason(num: u16) -> Option<&'static str> {
            match num {
                $( $num => Some($phrase), )+
                _ => None
            }
        }
    }
}

status_codes! {
    (200, OK, "OK");
    (201, CREATED, "Created");
    (404, NOT_FOUND, "Not Found");
}
