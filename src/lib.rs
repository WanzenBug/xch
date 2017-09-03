// Copyright 2017 Moritz Wanzenböck.
//
// Licensed under the MIT License <LICENSE or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to those terms.

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate cfg_if;

use std::path;

mod platform;
mod error;
pub use error::{Error};

/// Exchange the content of the objects pointed to by the two paths.
///
/// This can be used to swap the content of two files, but it also works with directories.
/// This operation is atomic, meaning if the content at one path changed, the other path will
/// also have changed. If the operation can't be done atomically, it will fail.
pub fn xch<A: AsRef<path::Path>, B: AsRef<path::Path>>(path1: A, path2: B) -> error::Result<()> {
    platform::xch(path1, path2)
}
