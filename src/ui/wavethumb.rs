#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

// use iced::advanced::{image::Image};
use iced::{Element, Length, Alignment};
use iced::widget::{text, image, mouse_area, column, horizontal_rule};

use std::path::PathBuf;

use super::Message;
use crate::sound_object::AudioContent;


fn trim_text(txt: String, limit: usize) -> String {
    if txt.len() > limit {
        let limit = limit - 6;
        format!("{} ...", &txt[..limit])
    } else {
        txt
    }
}

#[derive(Debug, Clone)]
pub struct WaveThumb {
    id: String,
    audio: AudioContent,
    img_file: PathBuf,
    playing: bool,
}

impl WaveThumb {
    pub fn new(
        id: String,
        audio: AudioContent,
        img_file: PathBuf,
    ) -> Self {
        WaveThumb { id, audio, img_file, playing: false }
    }
}

impl<'a> From<WaveThumb> for Element<'a, Message>
where Message: Clone + 'a,
{
    fn from(thumb: WaveThumb) -> Self {
        let ifile_name = thumb.img_file.to_string_lossy();
        let sfile_name = thumb.id.clone();
        column![
            mouse_area(
                image(&thumb.img_file)
                    .width(320)
            ).on_press(Message::Toggle(thumb.id)),
            text(trim_text(sfile_name, 38))
                .width(Length::Fill)
                .align_x(Alignment::Center),
            horizontal_rule(1)
        ]
        .width(Length::Shrink)
        .spacing(4)
        .into()
    }
}
