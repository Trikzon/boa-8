use std::fs::File;
use std::io;
use std::io::prelude::*;

pub fn read_binary_file(path: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;

    Ok(buffer)
}
