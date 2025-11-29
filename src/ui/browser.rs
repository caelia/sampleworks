#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

// use anyhow::{Result, Error, anyhow};

use iced::widget::{Row, row, scrollable, center, container};
use iced::{Element, Result, Task, Color, Length};

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::time::Duration;
use std::thread::sleep;

use super::wavethumb;
use super::Message;
use crate::sound_object::SoundObject;
use crate::project::Project;
use crate::messaging::{ACReq, ACRsp, TxWrapper, RxWrapper};


pub struct SampleBrowser {
    project: Rc<Project>,
    req_tx: Rc<TxWrapper<ACReq>>,
    rsp_rx: Rc<RxWrapper<ACRsp>>,
    playing: Option<String>,
}

impl SampleBrowser {
    pub fn new(project: Rc<Project>,
            tx: TxWrapper<ACReq>,
            rx: RxWrapper<ACRsp>)
            -> Self {
        Self {
            project,
            req_tx: Rc::new(tx),
            rsp_rx: Rc::new(rx),
            playing: None,
        }
    }

    pub fn view(&self) -> Element<Message> {
        let thumbs = self.project.objects.iter().map(|(id, obj)| {
            let img_file = obj.thumbnail().clone().unwrap();
            wavethumb::WaveThumb::new(
                id.clone(), obj.content().clone(), img_file
            ).into()
        }).collect::<Vec<_>>();

        scrollable(
            container(
                Row::from_vec(thumbs)
                    .spacing(17)
                    .padding(10)
                    // .width(Length::Fill)
                    .wrap()
            )
            .center_x(Length::Fill)
        )
        .width(Length::Fill)
        .into()
    }

    pub fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::Toggle(id) => {
                let (stop, start) = match &self.playing {
                    Some(id_) if *id_ == id => (true, false),
                    Some(id_) => (true, true),
                    None => (false, true),
                };
                if stop {
                    self.req_tx.send(ACReq::Stop);
                    self.playing = None;
                }
                if start {
                    if let Some(audio) = self.project.get_audio(&id) {
                        // let snd_file = obj.content.clone();
                        self.req_tx.send(ACReq::Audition(audio));
                        self.playing = Some(id);
                    }
                }
            },
            _ => (),
        }
        Task::none()
    }
}
