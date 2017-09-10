extern crate uuid;

use std::io;
use std::fs;
use std::path;

use error::{Result, Error};

/// Exchange files/directories by non-atomic renames.
///
/// This tries to swap files or directories by calling `std::fs::rename` 3 times.
/// 1. Rename path1 to a temporary path
/// 2. Rename path2 to path1
/// 3. Rename the temporary path to path2
///
/// This does not happen atomically. This means there is a time frame were path1 does not exist. Or
/// path1 does exists but path2 does not.
///
/// On error the changes are rolled back, if possible.
pub fn xch<A: AsRef<path::Path>, B: AsRef<path::Path>>(path1: A, path2: B) -> Result<()> {
    let path1 = path1.as_ref();
    let path2 = path2.as_ref();

    let one_parent = path1.parent()
        .ok_or_else::<Error, _>(|| format!("Could not find parent directory for {}", path1.display()).into())
        .or_else(|_|
            path2.parent()
                .ok_or_else::<Error, _>(|| format!("Could not find parent directory for {}", path2.display()).into())
        )?;
    let unique_name = format!("{}", uuid::Uuid::new_v4().hyphenated());
    let temp_name = one_parent.join(unique_name);

    let mut transaction = Transaction::new();
    transaction.record_rename(path1, &temp_name);
    transaction.record_rename(path2, path1);
    transaction.record_rename(&temp_name, path2);
    transaction.commit()
}

/// Structure to hold information about a rename to execute.
struct RenameCmd<'a> {
    from: &'a path::Path,
    to: &'a path::Path,
}

impl<'a> RenameCmd<'a> {
    /// Execute the rename represented by this struct.
    fn exec(&self) -> io::Result<()> {
        fs::rename(self.from, self.to)
    }

    /// Rollback the changes created by this struct.
    fn rollback(self) -> io::Result<()> {
        fs::rename(self.to, self.from)
    }
}

/// Represents a new file system transaction.
struct Transaction<'a> {
    to_exec: Vec<RenameCmd<'a>>,
    successful_exec: Vec<RenameCmd<'a>>,
}

impl<'a> Transaction<'a> {
    /// Create a new transaction.
    fn new() -> Transaction<'a> {
        Transaction {
            to_exec: Vec::new(),
            successful_exec: Vec::new(),
        }
    }

    /// Record a rename that should happen on commit.
    fn record_rename(&mut self, from: &'a path::Path, to: &'a path::Path) {
        self.to_exec.push(RenameCmd {
            from,
            to,
        })
    }

    /// Rollback all changes caused by this transaction.
    fn rollback(mut self) -> Result<()> {
        loop {
            if let Some(cmd) = self.successful_exec.pop() {
                if let Err(e) = cmd.rollback() {
                    return Err(e.into());
                }
            } else {
                return Ok(());
            }
        }
    }

    /// Commit all recorded changes.
    ///
    /// On success, all renames are executed as they were recorded.
    /// On error, this calls `rollback()` and tries to undo all changes.
    fn commit(mut self) -> Result<()> {
        let res = {
            let mut drain = self.to_exec.drain(..);
            loop {
                if let Some(cmd) = drain.next() {
                    let ex = cmd.exec();
                    if let Err(e) = ex {
                        break Err(e);
                    } else {
                        self.successful_exec.push(cmd);
                    }
                } else {
                    break Ok(());
                }
            }
        };

        if let Err(rename_err) = res {
            let rollback_res = self.rollback();
            if let Err(e) = rollback_res {
                let rename_err: Error = rename_err.into();
                Err(rename_err.chain_err(|| e))
            } else {
                Err(rename_err.into())
            }
        } else {
            res.map_err(Into::into)
        }
    }
}
