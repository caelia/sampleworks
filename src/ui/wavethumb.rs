#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

// use iced::advanced::{image::Image};
use iced::{Element, Task, Result};
use iced::advanced::Widget;
use iced::widget::{text, image, mouse_area, row, Column, column};

use std::path::PathBuf;
use std::rc::Rc;

use crate::messaging::{ACReq, ACRsp, TxWrapper};

#[derive(Debug, Clone)]
pub enum Message {
    Play(PathBuf),
    Stop,
    Toggle,
}

#[derive(Debug, Clone)]
pub struct WaveThumb<W: Widget<_, _, _> {
    snd_file: PathBuf,
    img_file: PathBuf,
    container: Option<W>,
    // req_tx: Rc<TxWrapper<ACReq>>,
    playing: bool,
}

impl WaveThumb {
    pub fn new(
        snd_file: PathBuf,
        img_file: PathBuf,
        // tx: TxWrapper<ACReq>,
    ) -> Self {
        // WaveThumb { snd_file, img_file, req_tx: Rc::new(tx), playing: false }
        WaveThumb { snd_file, img_file, container: None, playing: false }
    }

    pub fn view(&mut self) -> Element<Message> {
        let ifile_name = self.img_file.to_string_lossy();
        let sfile_name = self.snd_file.file_name().unwrap().to_string_lossy();
        let container = column![
            mouse_area(
                image(&self.img_file)
                    .width(320),
            ).on_press(Message::Toggle),
            text(sfile_name)
        ]
        .spacing(8);
        self.container = Some(container.clone());
        container.into()
    }

    pub fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::Toggle => {
                self.toggle();
            },
            _ => ()
        }
        println!("WaveThumb::update - {}", self.playing);
        Task::none()
    }

    fn toggle(&mut self) -> bool {
        let new_state = !self.playing;
        self.playing = new_state;
        new_state
    }
}

impl<'a, Message> From<WaveThumb> for Element<'a, Message>
where Message: Clone + 'a,
{
    fn from(thumb: WaveThumb) -> Self {
        thumb.container.into()
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
