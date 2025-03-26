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
    (AIM, A_IM, b"A-IM");
    (Accept, ACCEPT, b"Accept");
    (AcceptCharset, ACCEPT_CHARSET, b"Accept-Charset");
    (AcceptDatetime, ACCEPT_DATETIME, b"Accept-Datetime");
    (AcceptEncoding, ACCEPT_ENCODING, b"Accept-Encoding");
    (AcceptLanguage, ACCEPT_LANGUAGE, b"Accept-Language");
    (AcceptCtrlSeqMethod, ACCEPT_CTRL_SEQ_METHOD, b"Accept-Control-Sequence-Method");
    (AcceptCtrlSeqHeaders, ACCEPT_CTRL_SEQ_HEADERS, b"Accept-Control-Sequence-Headers");
    (Authorization, AUTHORIZATION, b"Authorezation");
    (CacheControl, CACHE_CONTROL, b"Cache-Control");
    (Connection, CONNECTION, b"Connection");
    (ContentEncoding, CONTENT_ENCODING, b"Content-Encoding");
    (ContentLength, CONTENT_LENGTH, b"Content-Length");
    (ContentMD5, CONTENT_MD5, b"Content-MD5");
    (ContentType, CONTENT_TYPE, b"Content-Type");
    (Cookie, COOKIE, b"Cookie");
    (CfVisitor, CF_VISITOR, b"Cf-Visitor");
    (CfConnectionIp, CF_CONNECTION_IP, b"Cf-Connection-Ip");
    (CfIpcountry, CF_IPCOUNTRY, b"Cf-Ipcountry");
    (CfRay, CF_RAY, b"Cf-Ray");
    (Date, DATE, b"Date");
    (Expect, EXPECT, b"Expect");
    (Forwarded, FORWARDED, b"Forwarded");
    (From, FROM, b"From");
    (Host, HOST, b"Host");
    (HTTP2Settings, HTTP2_SETTINGS, b"HTTP2-Settings");
    (IfMatch, IF_MATCH, b"If-Match");
    (IfModifiedSince, IF_MODIFIED_SINCE, b"If-Modified-Since");
    (IfNoneMatch, IF_NONE_MATCH, b"If-None-Match");
    (IfRange, IF_RANGE, b"If-Range");
    (IfUnmodifiedSince, IF_UNMODIFIED_SINCE, b"If-Unmodified-Since");
    (MaxForwards, MAX_FORWARDS, b"Max-Forwards");
    (Origin, ORIGIN, b"Origin");
    (Pragma, PRAGMA, b"Pragma");
    (Prefer, PREFER, b"Prefer");
    (ProxyAuthorization, PROXY_AUTHORIZATION, b"Proxy-Authorization");
    (Priority, PRIORITY, b"Priority");
    (Range, RANGE, b"Range");
    (Referer, REFERER, b"Referer");
    (TE, TE, b"TE");
    (Trailer, TRAILER, b"Trailer");
    (TransferEncoding, TRANSFER_ENCODING, b"Transfer-Encoding");
    (UserAgent, USER_AGENT, b"User-Agent");
    (Upgrade, UPGRADE, b"Upgrade");
    (Via, VIA, b"Via");
    (SecChUa, SEC_CH_UA, b"sec-ch-ua");
    (SecChUaMobile, SEC_CH_UA_MOBILE, b"sec-ch-ua-mobile");
    (SecChUaPlatform, SEC_CH_UA_PLATFORM, b"sec-ch-ua-platform");
    (SecFetchDest, SEC_FETCH_DEST, b"Sec-Fetch-Dest");
    (SecFetchMode, SEC_FETCH_MODE, b"Sec-Fetch-Mode");
    (SecFetchSite, SEC_FETCH_SITE, b"Sec-Fetch-Site");
    (SecFetchUser, SEC_FETCH_USER, b"Sec-Fetch-User");
    (SecGpc, SEC_GPC, b"Sec-GPC");
    (UpgradeInsecureRequests, UPGRADE_INSECURE_REQUESTS, b"Upgrade-Insecure-Requests");
    (XForwardedProto, X_FORWARDED_PROTO, b"X-Forwarded-Proto");
    (XHTTPS, X_HTTPS, b"X-HTTPS");
}
