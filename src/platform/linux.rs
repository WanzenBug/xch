use errno;
use std::{
    error,
    fmt,
    path,
    ffi,
    os,
};

use crate::error::Result;
extern {
    fn linux_xch_syscall(path1: *const os::raw::c_char, path2: *const os::raw::c_char) -> os::raw::c_long;
}

pub fn xch<A: AsRef<path::Path>, B: AsRef<path::Path>>(path1: A, path2: B) -> Result<()> {
    use std::os::unix::ffi::OsStrExt;

    let path1 = path1.as_ref().as_os_str().as_bytes();
    let path2 = path2.as_ref().as_os_str().as_bytes();
    let path1 = ffi::CString::new(path1).expect("path cannot contain null byte");
    let path2 = ffi::CString::new(path2).expect("path cannot contain null byte");
    let ret = unsafe {
        linux_xch_syscall(path1.as_ptr(), path2.as_ptr())
    };
    if ret == 0 {
        Ok(())
    } else {
        Err(PlatformError(errno::errno()).into())
    }
}

#[derive(Debug)]
pub struct PlatformError(errno::Errno);

impl fmt::Display for PlatformError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.0)
    }
}

impl error::Error for PlatformError {
    fn description(&self) -> &str {
        "OSError"
    }
}
