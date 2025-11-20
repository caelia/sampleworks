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
    pub content: PathBuf,    // audio file
    pub thumbnail: Option<PathBuf>,  // image file
}

impl SoundObject {
    fn new(content: PathBuf) -> Self {
         Self { content, thumbnail: None }
    }
}

pub fn snd_object(fname: PathBuf) -> Result<(String, SoundObject)> {
    let id = match std::fs::read(&fname) {
        Ok(bytes) => b3hash(bytes.as_slice()).to_string(),
        Err(e) => return Err(anyhow!(e)),
    };
    Ok((id, SoundObject::new(fname)))
}

/*
pub enum AudioState<T> {
    Stopped,
    Paused,
    Running(JoinHandle<T>),
}
*/
