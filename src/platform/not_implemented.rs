use crate::error::{Error, Result};
use std::{
    error,
    fmt,
    path,
};

pub fn xch<A: AsRef<path::Path>, B: AsRef<path::Path>>(_path1: A, _path2: B) -> Result<()> {
    Err(Error::NotImplemented)
}

#[derive(Debug)]
pub struct PlatformError;

impl fmt::Display for PlatformError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "GenericPlatformError")
    }
}

impl error::Error for PlatformError {
    fn cause(&self) -> Option<&error::Error> {
        None
    }

    fn description(&self) -> &str {
        "Generic error because atomic methods are not implemented for the current platform"
    }
}
