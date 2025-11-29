use blake3::hash as b3hash;
use anyhow::{anyhow, Result};
use rodio::{Decoder, source::Source};

use std::path::PathBuf;
use std::time::Duration;
use std::fs::File;

use crate::common::Channels;

pub trait SoundObject {
    fn hash(&self) -> &String;
    fn content(&self) -> &PathBuf;
    fn thumbnail(&self) -> &Option<PathBuf>;
    fn len(&self) -> Result<Duration>;
    fn sample_rate(&self) -> Result<u16>;
    fn channels(&self) -> Result<Channels>;
}

#[derive(Debug, Clone)]
pub struct FileObject {
    pub hash: String,
    pub content: PathBuf,    // audio file
    pub thumbnail: Option<PathBuf>,  // image file
    length: Option<Duration>,
    sample_rate: Option<u32>,
    channels: Option<Channels>,
}

impl FileObject {
    pub fn new(content: PathBuf) -> Result<Self> {
        let hash = match std::fs::read(&content) {
            Ok(bytes) => b3hash(bytes.as_slice()).to_string(),
            Err(e) => return Err(anyhow!(e)),
        };
        Ok(Self {
            hash,
            content,
            thumbnail: None,
            length: None,
            sample_rate: None,
            channels: None,
        })
    }

    fn set_audio_data(&mut self) -> Result<()> {
        let f = File::open(self.content.clone())?;
        let src = Decoder::try_from(f)?;
        self.length = src.total_duration();
        self.sample_rate = Some(src.sample_rate());
        self.channels = Some(match src.channels() {
            1 => Channels::Mono,
            2 => Channels::Stereo,
            c => Channels::Other(c as u16),
        });
        Ok(())
    }
}

impl SoundObject for FileObject {
    fn hash(&self) -> &String {
        &self.hash
    }
    fn content(&self) -> &PathBuf {
        &self.content
    }
    fn thumbnail(&self) -> &Option<PathBuf> {
        &self.thumbnail
    }
    fn len(&self) -> Result<Duration> {
        match self.length {
            Some(dur) => Ok(dur),
            None => {
                self.ref_mut().set_audio_data();
                match self.length {
                    Some(dur) => Ok(dur),
                    None => Err(anyhow!("Can't get duration.")),
                }
            }
        }
    }
}
