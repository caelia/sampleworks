#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

// use anyhow::{Result, Error, anyhow};

use iced::widget::{Row, row};
use iced::{Element, Result, Task};

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
// use std::sync::mpsc::{Sender, Receiver};
use std::time::Duration;
use std::thread::sleep;

use super::wavethumb;
use crate::messaging::{ACReq, ACRsp, TxWrapper, RxWrapper};

#[derive(Debug, Clone)]
pub enum Message {
    NewMap(Vec<(PathBuf, PathBuf)>),
    ThumbMsg(wavethumb::Message),
}

pub struct SampleBrowser {
    // Vec of (img_file, snd_file)
    file_map: Vec<(PathBuf, PathBuf)>,
    req_tx: Rc<TxWrapper<ACReq>>,
    rsp_rx: Rc<RxWrapper<ACRsp>>,
}

impl SampleBrowser {
    pub fn new(file_map: Vec<(PathBuf, PathBuf)>,
            tx: TxWrapper<ACReq>, rx: RxWrapper<ACRsp>)
            -> Self {
        Self {
            file_map,
            req_tx: Rc::new(tx),
            rsp_rx: Rc::new(rx)
        }
    }

    pub fn view(&self) -> Element<Message> {
        let thumbs = self.file_map.into_iter().map(|(img_file, snd_file)| {
            wavethumb::WaveThumb::new(snd_file, img_file)
        }).collect::<Vec<_>>();

        Row::from_vec(thumbs)
            .spacing(12)
            .into()
    }

    pub fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::NewMap(fmap) => self.file_map = fmap,
            _ => ()
        }
        Task::none()
    }
}


//         let tx = self.req_tx.clone();
//         let _ = app.connect_activate(move |app| {
//             let window = ApplicationWindow::builder()
//                 .application(app)
//                 .default_width(840)
//                 .default_height(720)
//                 .title("SampleWorks Prototype")
//                 .build();

//             let cssp = gtk::CssProvider::new();
//             cssp.load_from_data(CSS);
//             gtk::style_context_add_provider_for_display(
//                 &gtk::gdk::Display::default().expect("SMUCK!"),
//                 &cssp,
//                 gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
//             );

//             let fbox = gtk::FlowBox::builder()
//                 .orientation(Orientation::Horizontal)
//                 .valign(gtk::Align::Start)
//                 .halign(gtk::Align::Center)
//                 .max_children_per_line(9)
//                 .min_children_per_line(2)
//                 .selection_mode(gtk::SelectionMode::Multiple)
//                 .build();

//             for (img_file, snd_file) in &file_map {
//                 let img = Picture::builder()
//                     .hexpand(false)
//                     .vexpand(true)
//                     .content_fit(ContentFit::Contain)
//                     .build();
//                 img.set_filename(Some(img_file));
//                 let ectrl_ck = gtk::GestureClick::new();
//                 ectrl_ck.connect_released(clone!(
//                     #[strong] tx,
//                     move |_, _, _, _| {
//                         let _ = tx.send(ACReq::Stop);
//                     }
//                 ));
//                 let ectrl_lp = gtk::GestureLongPress::new();
//                 ectrl_lp.connect_pressed(clone!(
//                     #[strong] snd_file,
//                     #[strong] tx,
//                     move |_, _, _| {
//                         let _ = tx.send(ACReq::Audition(snd_file.clone()));
//                     }
//                 ));
//                 img.add_controller(ectrl_ck);
//                 img.add_controller(ectrl_lp);
//                 fbox.insert(&img, -1);
//             }

//             let scrolled = gtk::ScrolledWindow::builder()
//                 .hscrollbar_policy(gtk::PolicyType::Never)
//                 .vscrollbar_policy(gtk::PolicyType::Always)
//                 .min_content_width(400)
//                 .child(&fbox)
//                 .build();

//             window.set_child(Some(&scrolled));
//             window.present();
//         });

//         app.run()
//     }
// }
