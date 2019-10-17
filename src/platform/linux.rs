use errno;
use libc;
use std::{
    error,
    fmt,
    mem,
    path,
};
use syscall;

use crate::error::Result;

pub fn xch<A: AsRef<path::Path>, B: AsRef<path::Path>>(path1: A, path2: B) -> Result<()> {
    use std::os::unix::ffi::OsStrExt;

    let path1 = path1.as_ref().as_os_str().as_bytes();
    let path2 = path2.as_ref().as_os_str().as_bytes();
    let mut path1_vec = Vec::with_capacity(path1.len() + 1);
    let mut path2_vec = Vec::with_capacity(path2.len() + 1);
    path1_vec.extend_from_slice(path1);
    path1_vec.push(0);
    path2_vec.extend_from_slice(path2);
    path2_vec.push(0);
    let flag = libc::RENAME_EXCHANGE as usize;

    let ret = unsafe {
        let cwd = libc::AT_FDCWD as usize;
        let path1_ptr = mem::transmute::<_, usize>(path1_vec.as_ptr());
        let path2_ptr = mem::transmute::<_, usize>(path2_vec.as_ptr());
        syscall::syscall5(syscall::nr::RENAMEAT2, cwd, path1_ptr, cwd, path2_ptr, flag)
    };
    if ret == 0 {
        Ok(())
    } else {
        Err(PlatformError(errno::Errno(((usize::max_value() - ret) + 1) as i32)).into())
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
