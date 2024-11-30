use std::convert::TryFrom;
use std::fmt;
use std::hash::Hash;

use crate::error::InvalidHeaderName;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HeaderName {
    inner: StandardHeader,
}

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
    (AcceptLanguage, ACCEPT_LANGUAGE, b"Accept-Language");
    (Authorization, AUTHORIZATION, b"Authorezation");
    (Connection, CONNECTION, b"Connection");
    (ContentEncoding, CONTENT_ENCODING, b"Content-Encoding");
    (CacheControl, CACHE_CONTROL, b"Cache-Control");
    (Cookie, COOKIE, b"Cookie");
    (Host, HOST, b"Host");
    (Origin, ORIGIN, b"Origin");
    (Referer, REFERER, b"Referer");
    (SecChUa, SEC_CH_UA, b"sec-ch-ua");
    (SecChUaMobile, SEC_CH_UA_MOBILE, b"sec-ch-ua-mobile");
    (SecChUaPlatform, SEC_CH_UA_PLATFORM, b"sec-ch-ua-platform");
    (SecFetchDest, SEC_FETCH_DEST, b"Sec-Fetch-Dest");
    (SecFetchMode, SEC_FETCH_MODE, b"Sec-Fetch-Mode");
    (SecFetchSite, SEC_FETCH_SITE, b"Sec-Fetch-Site");
    (SecFetchUser, SEC_FETCH_USER, b"Sec-Fetch-User");
    (SecGpc, SEC_GPC, b"Sec-GPC");
    (UpgradeInsecureRequests, UPGRADE_INSECURE_REQUESTS, b"Upgrade-Insecure-Requests");
}
