#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use blake3::hash as b3hash;
use anyhow::{anyhow, Result};

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
pub enum Channels {
    Mono,
    Stereo,
    Other(u16),
    Unknown,
}
