#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

// use iced::advanced::{image::Image};
use iced::{Element, Task, Result};
use iced::advanced::Widget;
use iced::widget::{text, image, mouse_area, row, Column, column};

use std::path::PathBuf;
use std::rc::Rc;
use std::borrow::Cow;

use crate::messaging::{ACReq, ACRsp, TxWrapper};
use super::Message;


#[derive(Debug, Clone)]
pub struct WaveThumb {
    id: String,
    snd_file: PathBuf,
    img_file: PathBuf,
    playing: bool,
}

impl WaveThumb {
    pub fn new(
        id: String,
        snd_file: PathBuf,
        img_file: PathBuf,
    ) -> Self {
        WaveThumb { id, snd_file, img_file, playing: false }
    }

    pub fn filename(&self) -> Cow<str> {
        self.snd_file.file_name().unwrap().to_string_lossy()
    }
}

impl<'a> From<WaveThumb> for Element<'a, Message>
where Message: Clone + 'a,
{
    fn from(thumb: WaveThumb) -> Self {
        let ifile_name = thumb.img_file.to_string_lossy();
        // let sfile_name = thumb.snd_file.clone().file_name().unwrap().to_string_lossy();
        let sfile_name = thumb.filename().into_owned();
        column![
            mouse_area(
                image(&thumb.img_file)
                    .width(320)
            ).on_press(Message::Toggle(thumb.id)),
            text(sfile_name)
        ]
        .spacing(8)
        .into()
    }
}

/*
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
*/
