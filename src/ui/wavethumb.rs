#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

// use iced::advanced::{image::Image};
use iced::Element;
use iced::widget::{text, image, mouse_area, column};

use std::path::PathBuf;
use std::rc::Rc;

use crate::messaging::{ACReq, ACRsp, TxWrapper};

#[derive(Debug, Clone)]
pub enum Message {
    Play(PathBuf),
    Stop
}

#[derive(Debug, Clone)]
pub struct WaveThumb {
    snd_file: PathBuf,
    img_file: PathBuf,
    // req_tx: Rc<TxWrapper<ACReq>>,
    playing: bool,
}

impl WaveThumb {
    fn new(
        snd_file: PathBuf,
        img_file: PathBuf,
        // tx: TxWrapper<ACReq>,
    ) -> Self {
        // WaveThumb { snd_file, img_file, req_tx: Rc::new(tx), playing: false }
        WaveThumb { snd_file, img_file, playing: false }
    }

    fn toggle(&mut self) -> bool {
        let new_state = !self.playing;
        self.playing = new_state;
        new_state
    }
}

pub fn view(thumb: &mut WaveThumb) -> Element<Message> {
    let name = match thumb.snd_file.file_name() {
        Some(nm) => nm.display().to_string(),
        None => "Unknown".to_string(),
    };
    mouse_area(
        column![
            image(thumb.img_file.clone()),
            text(name)
        ]
    )
    .on_press(
        if thumb.toggle() {
            Message::Play(thumb.snd_file.clone())
        } else {
            Message::Stop
        }
    )
    .into()
}

