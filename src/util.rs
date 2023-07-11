use blake3;
use anyhow::{anyhow, Result};
use std::path::PathBuf;
use std::fs::read;
use std::fmt::format;

pub fn file_hash(path: &PathBuf) -> Result<String> {
    match read(path) {
        Ok(bytes) => Ok(format!("{}", blake3::hash(&bytes))),
        Err(e) => Err(anyhow!(e)),
    }
}
