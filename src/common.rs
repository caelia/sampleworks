use serde::{Deserialize, Serialize};
use std::thread::JoinHandle;

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub enum AudioFormat {
    WAV,
    FLAC,
    OGG,
    MP3,
    AAC
}

/*
pub enum AudioState<T> {
    Stopped,
    Paused,
    Running(JoinHandle<T>),
}
*/
