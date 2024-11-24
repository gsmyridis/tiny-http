use std::convert::TryFrom;
use std::fmt;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HeaderName {
    inner: StandardHeader,
}

/// Error for bytes that don't correspond to a valid header.
#[derive(Debug)]
pub struct InvalidHeaderName;

macro_rules! standard_headers {
    (
        $(
            ($konst:ident, $uppercase:ident, $name_bytes:expr);
        )+
    ) => {

        #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
        enum StandardHeader {
            $($konst,)+
        }


        impl HeaderName {

            $(
                pub const $uppercase: HeaderName = HeaderName { inner: StandardHeader::$konst };
            )+

            /// Returns the header name as a static string.
            pub fn as_str(&self) -> &'static str {
                match &self.inner {
                    $(
                        StandardHeader::$konst => unsafe { std::str::from_utf8_unchecked( $name_bytes )},
                    )+
                }
            }


            /// Creates a `HeaderName` from a string reference.
            pub fn from_bytes(bytes: &[u8]) -> Option<HeaderName> {
                match bytes {
                    $( $name_bytes => Some(HeaderName { inner: StandardHeader::$konst }),)+
                    _ => None,
                }
            }
        }
    }
}

impl<const N: usize> TryFrom<&[u8; N]> for HeaderName {
    type Error = InvalidHeaderName;

    fn try_from(bytes: &[u8; N]) -> Result<Self, Self::Error> {
        HeaderName::from_bytes(bytes).ok_or(InvalidHeaderName)
    }
}

impl TryFrom<&[u8]> for HeaderName {
    type Error = InvalidHeaderName;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        HeaderName::from_bytes(bytes).ok_or(InvalidHeaderName)
    }
}

impl TryFrom<&str> for HeaderName {
    type Error = InvalidHeaderName;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        TryFrom::try_from(s.as_bytes())
    }
}

impl fmt::Display for HeaderName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

standard_headers! {
    (ContentType, CONTENT_TYPE, b"Content-Type");
    (ContentLength, CONTENT_LENGTH, b"Content-Length");
    (UserAgent, USER_AGENT, b"User-Agent");
    (Accept, ACCEPT, b"Accept");
    (AcceptEncoding, ACCEPT_ENCODING, b"Accept-Encoding");
    (ContentEncoding, CONTENT_ENCODING, b"Content-Encoding");
    (Host, HOST, b"Host");
}
