#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use blake3::hash as b3hash;
use anyhow::{anyhow, Result};
use rodio::{Decoder, source::Source};

use std::path::PathBuf;
use std::time::Duration;
use std::fs::File;
use std::sync::LazyLock;
use std::ops::Fn;

use crate::common::Channels;

#[derive(Debug, Clone)]
pub enum AudioContent {
    File(PathBuf),
}

pub trait SoundObject {
    fn hash(&self) -> &String;
    fn content(&self) -> AudioContent;
    fn thumbnail(&self) -> &Option<PathBuf>;
    fn set_thumbnail(&mut self, path: PathBuf);
    fn len(&self) -> Result<Duration>;
    fn sample_rate(&self) -> Result<u32>;
    fn channels(&self) -> Result<Channels>;
}

type AudioData = (Option<Duration>, u32, Channels);
#[derive(Debug)]
pub struct FileObject {
    pub hash: String,
    pub content: PathBuf,    // audio file
    pub thumbnail: Option<PathBuf>,  // image file
    audio_data: LazyLock<AudioData, Box<dyn Fn() -> AudioData>>,
} 

impl FileObject {
    pub fn new(content: PathBuf) -> Result<Self> {
        let hash = match std::fs::read(&content) {
            Ok(bytes) => b3hash(bytes.as_slice()).to_string(),
            Err(e) => return Err(anyhow!(e)),
        };
        let content_ = content.clone();
        Ok(Self {
            hash,
            content,
            thumbnail: None,
            audio_data:
                 LazyLock::new(Box::new(move || {
                    let f = File::open(&content_).unwrap();
                    let src = Decoder::try_from(f).unwrap();
                    (
                        src.total_duration(),
                        src.sample_rate() as u32,
                        match src.channels() {
                            1 => Channels::Mono,
                            2 => Channels::Stereo,
                            c => Channels::Other(c),
                        }
                    )
                })),
        })
    }
}

impl SoundObject for FileObject {
    fn hash(&self) -> &String {
        &self.hash
    }

    fn content(&self) -> AudioContent {
        let content = self.content.clone();
        AudioContent::File(content)
    }

    fn thumbnail(&self) -> &Option<PathBuf> {
        &self.thumbnail
    }

    fn set_thumbnail(&mut self, path: PathBuf) {
        self.thumbnail = Some(path)
    }

    fn len(&self) -> Result<Duration> {
        match self.audio_data.0 {
            Some(dur) => Ok(dur),
            None => Err(anyhow!("Can't detect file duration.")),
        }
    }

    fn sample_rate(&self) -> Result<u32> {
        Ok(self.audio_data.1)
    }

    fn channels(&self) -> Result<Channels> {
        Ok(self.audio_data.2.clone())
    }
}
