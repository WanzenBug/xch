// Copyright 2017 Moritz Wanzenböck.
//
// Licensed under the MIT License <LICENSE or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to those terms.
//! A library for exchanging paths
//!
//! This library provides a simple utility to swap files and/or directory content of two paths.
//! When possible, this is done in an atomic fashion, so that only the full changes are observable.
//!
//! Currently, atomic exchange is only supported on Windows and Linux.
#[macro_use]
extern crate cfg_if;

use std::path;

mod platform;
mod non_atomic;
mod error;

pub use error::Error;

/// Exchange the content of the objects pointed to by the two paths.
///
/// This can be used to swap the content of two files, but it also works with directories.
/// **This operation is atomic**, meaning if the content at one path changed, the other path will
/// also have changed. If the operation can't be done atomically, it will fail.
pub fn xch<A: AsRef<path::Path>, B: AsRef<path::Path>>(path1: A, path2: B) -> error::Result<()> {
    platform::xch(path1, path2)
}

/// Exchange the content of the object pointed to by the two paths.
///
/// This can be used to swap the content of two files, but it also works with directories.
/// **This operation may not be atomic**. If available, it will try to use the platform specific,
/// atomic operations. If they are not implemented, this will fallback to a non-atomic exchange.
pub fn xch_non_atomic<A: AsRef<path::Path>, B: AsRef<path::Path>>(path1: A, path2: B) -> error::Result<()> {
    let res: Result<_, _> = platform::xch(&path1, &path2);
    res.or_else(|e1|
        non_atomic::xch(&path1, &path2)
            .map_err(|e2| Error::ChainError(Box::new(e2), Box::new(e1)))
    )
}
