use serde::{Deserialize, Serialize};
use blake3::hash as b3hash;
use anyhow::{anyhow, Result};

use std::thread::JoinHandle;
use std::path::PathBuf;

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub enum AudioFormat {
    WAV,
    FLAC,
    OGG,
    MP3,
    AAC
}


#[derive(Debug, Clone)]
pub struct SoundObject {
    pub hash: String,
    pub content: PathBuf,    // audio file
    pub thumbnail: Option<PathBuf>,  // image file
}

impl SoundObject {
    pub fn new(content: PathBuf) -> Result<Self> {
        let hash = match std::fs::read(&content) {
            Ok(bytes) => b3hash(bytes.as_slice()).to_string(),
            Err(e) => return Err(anyhow!(e)),
        };
        Ok(Self { hash, content, thumbnail: None })
    }
}
