extern crate kernel32;
extern crate ktmw32;
extern crate winapi;

use std::{
    ffi::{OsStr, OsString},
    error,
    fmt,
    fs,
    mem,
    path,
    ptr::null_mut,
    slice
};

use crate::error::{Error, Result};

/// Exchange two paths on a Windows machine.
///
/// This is the windows implementation of xch. The implemetation uses the
/// [Transactional NTFS}(https://msdn.microsoft.com/en-us/library/windows/desktop/aa365008(v=vs.85).aspx)
/// feature on modern (>=Vista) Windows versions.
///
/// It works like this:
/// 1. Create a temporary file in the same directory as one of the parameters.
/// 2. Create a new transaction.
/// 3. Delete the temporary file (we only want it for the unique name).
/// 4. Move the first path to the path of the temporary file.
/// 5. Move the second path to the path of the first.
/// 6. Move the temporary path to the second path.
/// 7. Commit the transaction.
///
/// If any of the operations 3-7 fail, the changes will be rolled back and nothing will have changed.
pub fn xch<A: AsRef<path::Path>, B: AsRef<path::Path>>(file1: A, file2: B) -> Result<()> {
    let path1 = fs::canonicalize(file1.as_ref())?;
    let path2 = fs::canonicalize(file2.as_ref())?;

    let one_parent = path1.parent()
        .ok_or_else::<Error, _>(|| format!("Could not find parent directory for {}", path1.display()).into())
        .or_else(|_|
            path2.parent()
                .ok_or_else::<Error, _>(|| format!("Could not find parent directory for {}", path2.display()).into())
        )?;
    let temp_file_path = TempFile::new(one_parent)?;
    let transaction = Transaction::new()?;
    let transaction = transaction.delete_file(&temp_file_path)?;
    let transaction = transaction.move_file(&path1, &temp_file_path)?;
    let transaction = transaction.move_file(&path2, &path1)?;
    let transaction = transaction.move_file(&temp_file_path, &path2)?;
    transaction.commit()
}

/// Represents a windows transaction handle.
#[derive(Debug)]
struct Transaction(winapi::HANDLE);

impl Transaction {
    /// Create a new transaction.
    fn new() -> Result<Transaction> {
        let handle = unsafe {
            ktmw32::CreateTransaction(null_mut(), null_mut(), 0, 0, 0, 0, null_mut())
        };
        if handle == winapi::INVALID_HANDLE_VALUE {
            Err("Could not get transaction".into())
        } else {
            Ok(Transaction(handle))
        }
    }

    /// Try to commit the transaction.
    ///
    /// On failure, the transaction will be rolled back and an error is returned.
    /// On success, returns nothing
    fn commit(self) -> Result<()> {
        let res = as_win_error(unsafe { ktmw32::CommitTransaction(self.0) });
        if let Err(e) = res {
            self.rollback()?;
            Err(e)
        } else {
            res
        }
    }

    /// Rollback all changes recorded with this transaction.
    ///
    /// The rollback can fail. Since there is nothing that can be done about that, we don't return
    /// the transaction again.
    fn rollback(self) -> Result<()> {
        as_win_error(unsafe {
            ktmw32::RollbackTransaction(self.0)
        })
    }

    /// Move a file (or directory) from one path to another.
    ///
    /// This only works if the destination does not already exist. In case of success, returns
    /// another transaction. In case of failure rolls back the transaction and returns the error
    /// that caused the rollback.
    fn move_file<A: AsRef<path::Path>, B: AsRef<path::Path>>(self, from: A, to: B) -> Result<Self> {
        let from_encoded = to_wide_str(from.as_ref());
        let to_encoded = to_wide_str(to.as_ref());

        let handle = self.0;
        self.ok_or_rollback(as_win_error(unsafe {
            kernel32::MoveFileTransactedW(from_encoded.as_ptr(), to_encoded.as_ptr(), None, null_mut(), 0, handle)
        }))
    }

    /// Causes a rollback if the provided result is Err().
    ///
    /// In case of success return a new handle to this transaction so that it can be used to
    /// record more changes.
    fn ok_or_rollback(self, api_res: Result<()>) -> Result<Self> {
        if let Err(e) = api_res {
            self.rollback()?;
            Err(e)
        } else {
            api_res
                .map(|_| self)
        }
    }

    /// Delete a file (and only a file) at a given path.
    ///
    /// In case of success, returns another transaction. In case of failure rolls back the
    /// transaction and returns the error that caused the rollback.
    fn delete_file<A: AsRef<path::Path>>(self, to_delete: A) -> Result<Self> {
        let from_encoded = to_wide_str(to_delete.as_ref());

        let handle = self.0;
        self.ok_or_rollback(as_win_error(unsafe {
            kernel32::DeleteFileTransactedW(from_encoded.as_ptr(), handle)
        }))
    }
}

/// A temporary file on a Windows platform.
///
/// This stores the path to a temporary file. On drop, this file will be removed.
#[derive(Debug)]
struct TempFile(path::PathBuf);

impl TempFile {
    /// Create a new temporary file in the specified directory.
    fn new<A: AsRef<path::Path>>(dir_path: A) -> Result<Self> {
        use std::os::windows::ffi::OsStringExt;
        let mut out = Vec::with_capacity(winapi::MAX_PATH);
        let pre = to_wide_str("tmp");
        let dir = to_wide_str(dir_path.as_ref());
        if unsafe {
            kernel32::GetTempFileNameW(dir.as_ptr(), pre.as_ptr(), 0, out.as_mut_ptr())
        } != 0 {
            unsafe { out.set_len(winapi::MAX_PATH) };
            let n = out.iter().position(|&x| x == 0).ok_or_else(|| "Could not create tempfile")?;
            Ok(TempFile(OsString::from_wide(&out[..n]).into()))
        } else {
            let error = unsafe { kernel32::GetLastError() };
            Err(format!("Got Windows error code {:x}", error).into())
        }
    }
}

/// Get the underlying path to the TempFile.
impl AsRef<path::Path> for TempFile {
    fn as_ref(&self) -> &path::Path {
        self.0.as_ref()
    }
}

/// Clean up the temp file if it still exists.
impl Drop for TempFile {
    fn drop(&mut self) {
        if self.0.exists() {
            let _ = fs::remove_file(&self.0);
        }
    }
}

/// Encode a OsStr as a windows wide string with \\0 termination.
fn to_wide_str<O: AsRef<OsStr>>(s: O) -> Vec<u16> {
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;

    s.as_ref().encode_wide().chain(once(0)).collect()
}

/// Create a Result from a Windows ffi call that returns a Windows BOOL.
///
/// If the parameter equals the Windows FALSE constant, this will call `get_last_error()` to get
/// a meaningful error message from the system.
fn as_win_error(res: winapi::BOOL) -> Result<()> {
    if res != winapi::FALSE {
        Ok(())
    } else {
        Err(get_last_error().into())
    }
}

/// Gets a PlatformError from a Windows error.
///
/// This function calls `GetLastError()` to get the error code and uses this code to generate a
/// PlatformError with a Windows specific error message.
fn get_last_error() -> PlatformError {
    use std::os::windows::ffi::OsStringExt;
    let error = unsafe { kernel32::GetLastError() };

    let flags = winapi::FORMAT_MESSAGE_ALLOCATE_BUFFER | winapi::FORMAT_MESSAGE_FROM_SYSTEM | winapi::FORMAT_MESSAGE_IGNORE_INSERTS;
    let langid = winapi::LANG_USER_DEFAULT as u32;
    // This variable will store the location for the system allocated buffer
    let mut ptr: winapi::HLOCAL = null_mut();
    let size = unsafe {
        kernel32::FormatMessageW(flags, null_mut(), error, langid, mem::transmute::<_, *mut u16>(&mut ptr), 0, null_mut())
    };
    let msg = if size == 0 {
        "Unknown Error".into()
    } else {
        let slice = unsafe {
            slice::from_raw_parts::<u16>(ptr as *const u16, size as usize)
        };
        let msg = OsString::from_wide(slice).to_string_lossy().to_string();
        unsafe {
            kernel32::LocalFree(ptr)
        };
        msg
    };
    PlatformError(error, msg)
}

/// Represents a windows error
///
/// This stores the error code as well as an error message generated by Windows.
#[derive(Debug)]
pub struct PlatformError(winapi::DWORD, String);

impl fmt::Display for PlatformError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Windows Error [{}]: {}", self.0, self.1)
    }
}

impl error::Error for PlatformError {
    fn description(&self) -> &str {
        &self.1
    }
}
