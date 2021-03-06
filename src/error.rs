#[derive(Debug)]
pub enum Error {
    Fs(::std::io::Error),
    PlatformError(crate::platform::PlatformError),
    LogicError(String),
    ChainError(Box<Error>, Box<Error>),
    NotImplemented,
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Fs(_)
            | Error::PlatformError(_)
            | Error::ChainError(_, _) => "Could not exchange paths",
            |
            Error::LogicError(ref s) => s,
            Error::NotImplemented => "Not supported on this platform"
        }
    }

    fn source(&self) -> Option<&(dyn ::std::error::Error + 'static)> {
        match *self {
            Error::Fs(ref e) => Some(e),
            Error::PlatformError(ref e) => Some(e),
            Error::ChainError(ref e1, ref e2) => e1.source().or_else(|| e2.source()),
            _ => None,
        }
    }
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use ::std::error::Error;
        if let Some(cause) = self.source() {
            write!(f, "{}: {}", self.description(), cause)
        } else {
            write!(f, "{}", self.description())
        }
    }
}

impl From<::std::io::Error> for Error {
    fn from(e: ::std::io::Error) -> Self {
        Error::Fs(e)
    }
}

impl<'a> From<&'a str> for Error {
    fn from(s: &'a str) -> Self {
        Error::LogicError(s.to_string())
    }
}

impl From<String> for Error {
    fn from(e: String) -> Self {
        Error::LogicError(e)
    }
}

impl From<crate::platform::PlatformError> for Error {
    fn from(e: crate::platform::PlatformError) -> Self {
        Error::PlatformError(e)
    }
}

pub type Result<T> = ::std::result::Result<T, Error>;
