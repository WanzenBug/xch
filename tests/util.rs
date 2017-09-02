use std::path;
use std::fs;
use std::io::{Write, Read};
use std::error::Error;


pub fn ensure_file_content<A: AsRef<path::Path>>(path: A, content: &[u8]) -> Result<bool, Box<Error>> {
    let mut f = fs::File::open(path)?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;

    Ok(content == &buf[..])
}

pub fn create_file_with_content<A: AsRef<path::Path>>(path: A, content: &[u8]) -> Result<(), Box<Error>> {
    let mut f = fs::File::create(path)?;
    f.write_all(content).map_err(Into::into)
}