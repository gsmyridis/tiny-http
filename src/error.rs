use std::result;

#[derive(Debug)]
pub struct Error {
    #[allow(dead_code)]
    inner: ErrorKind,
}

macro_rules! create_errors {
    (
        $(
            ($error_kind:ident, $error:ident);
        )+
    ) => {

        $(
        #[derive(Debug)]
        pub struct $error;

        impl From<$error> for Error {
            fn from(err: $error) -> Error {
                Error{ inner: ErrorKind::$error_kind(err) }
            }
        }
        )+

        #[derive(Debug)]
        pub enum ErrorKind {
            $($error_kind($error),)+
        }

    }
}

create_errors! {
    (Method, InvalidMethod);
    (Uri, InvalidUri);
    (Version, InvalidVersion);
    (StatusCode, InvalidStatusCode);
    (Header, InvalidHeaderName);
    (Body, InvalidBody);
    (Connection, FailedConnection);
    (ErrorHandler, NoErrorHandler);
}

pub type Result<T> = result::Result<T, Error>;
