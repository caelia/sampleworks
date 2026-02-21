#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use iced::{Task, Element};
use iced_split::vertical_split;

use super::browser::SourceBrowser;
use super::organizer::Organizer;
use super::Message;
use crate::project::Project;
use crate::messaging::{ACReq, ACRsp, TxWrapper, RxWrapper};

use std::rc::Rc;

/*
enum Message {
    OnDrag(f32),
}
*/

pub struct MainWindow<'a> {
    split_at: f32,
    browser: SourceBrowser,
    organizer: Organizer,
    project: Rc<Project>,
    req_tx: TxWrapper<'a, ACReq>,
    rsp_rx: RxWrapper<'a, ACRsp>,
    playing: Option<String>,
}

impl<'a> MainWindow<'a> {
    pub fn new(
        project: Rc<Project>,
        req_tx: TxWrapper<'a, ACReq>,
        rsp_rx: RxWrapper<'a, ACRsp>,
    ) -> Self {
    let browser = SourceBrowser::new(project.clone());
        let organizer = Organizer::new();
        Self {
            split_at: 0.0,
            browser,
            organizer,
            project,
            req_tx,
            rsp_rx,
            playing: None,
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        vertical_split(
            self.browser.view(),
            self.organizer.view(),
            self.split_at,
            Message::OnDrag,
        )
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
                    if let Some(obj) = self.project.get_object(&id) {
                        let snd_file = obj.content.clone();
                        self.req_tx.send(ACReq::Audition(snd_file));
                        self.playing = Some(id);
                    }
                }
            },
            _ => (),
        }
        Task::none()
    }
}
