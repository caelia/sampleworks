#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, FlowBox, Image, Picture, ContentFit};
use gtk::glib;
use glib::source::SourceId;
use glib::clone;

use rodio::{Decoder, OutputStream, source::Source};
use anyhow::{Result, Error, anyhow};

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
// use std::sync::mpsc::{Sender, Receiver};
use std::time::Duration;
use std::thread::sleep;

use crate::messaging::{ACReq, ACRsp, TxWrapper, RxWrapper};

const CSS: &str = "
    picture {
        border: 1px solid #c0c0c0;
        padding: 0;
        margin: 0;
    }
";

pub struct SidWrapper {
    id: RefCell<Option<SourceId>>,
}

impl SidWrapper {
    fn new() -> Self {
        SidWrapper { id: RefCell::new(None) }
    }

    fn set(&self, sid: SourceId) {
        let _ = self.id.replace(Some(sid));
    }

    fn cancel(&self) {
        match self.id.take() {
            Some(sid) => sid.remove(),
            None => (),
        }
    }
}

pub struct SampleBrowser {
    snd_dir: PathBuf,
    img_dir: PathBuf,
    req_tx: Rc<TxWrapper<ACReq>>,
    rsp_rx: Rc<RxWrapper<ACRsp>>,
}

impl SampleBrowser {
    pub fn new(snd_dir: PathBuf, img_dir: PathBuf,
            tx: TxWrapper<ACReq>, rx: RxWrapper<ACRsp>)
            -> SampleBrowser {
        SampleBrowser {
            snd_dir,
            img_dir,
            req_tx: Rc::new(tx),
            rsp_rx: Rc::new(rx)
        }
    }

    pub fn run(&self, file_map: Vec<(PathBuf, PathBuf)>) -> glib::ExitCode {
    // pub fn run(&self) -> glib::signal::SignalHandlerId {
        let app = Application::builder()
            .application_id("org.sampleworks.SWPrototype")
            .build();

        /*
        let img_files = match self.img_dir.read_dir() {
            Ok(entries) => {
                entries.map(|result| {
                    match result {
                        Ok(entry) => Some(entry.path()),
                        Err(_) => None,
                    }
                }).collect::<Vec<Option<PathBuf>>>()
            },
            Err(_) => vec![],
        };
        */

        let tx = self.req_tx.clone();
        let _ = app.connect_activate(move |app| {
            let window = ApplicationWindow::builder()
                .application(app)
                .default_width(840)
                .default_height(720)
                .title("SampleWorks Prototype")
                .build();

            let cssp = gtk::CssProvider::new();
            cssp.load_from_data(CSS);
            gtk::style_context_add_provider_for_display(
                &gtk::gdk::Display::default().expect("SMUCK!"),
                &cssp,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
            );

            let fbox = gtk::FlowBox::builder()
                .valign(gtk::Align::Start)
                .max_children_per_line(9)
                .min_children_per_line(4)
                .selection_mode(gtk::SelectionMode::None)
                .build();

            // for file in &img_files {
            /*
                match file {
                    Some(path) => {
                        let img = Picture::for_filename(&path);
                        let ectrl_ck = gtk::GestureClick::new();
                        ectrl_ck.connect_released(clone!(
                            #[strong] tx,
                            move |_, _, _, _| {
                                let _ = tx.send(ACReq::Stop);
                            }
                        ));
                        let ectrl_lp = gtk::GestureLongPress::new();
                        ectrl_lp.connect_pressed(clone!(
                            #[strong] path,
                            #[strong] tx,
                            move |_, _, _| {
                                let _ = tx.send(ACReq::Audition(path.clone()));
                            }
                        ));
                        img.add_controller(ectrl_ck);
                        img.add_controller(ectrl_lp);
                        fbox.insert(&img, -1);
                    },
                    None => (),
                }
            */
            for (img_file, snd_file) in &file_map {
                // let img = Picture::for_filename(&img_file);
                let img = Picture::builder()
                    .hexpand(false)
                    .vexpand(true)
                    .content_fit(ContentFit::ScaleDown)
                    .build();
                img.set_filename(Some(img_file));
                let ectrl_ck = gtk::GestureClick::new();
                ectrl_ck.connect_released(clone!(
                    #[strong] tx,
                    move |_, _, _, _| {
                        let _ = tx.send(ACReq::Stop);
                    }
                ));
                let ectrl_lp = gtk::GestureLongPress::new();
                ectrl_lp.connect_pressed(clone!(
                    #[strong] snd_file,
                    #[strong] tx,
                    move |_, _, _| {
                        let _ = tx.send(ACReq::Audition(snd_file.clone()));
                    }
                ));
                img.add_controller(ectrl_ck);
                img.add_controller(ectrl_lp);
                fbox.insert(&img, -1);
            }

            let scrolled = gtk::ScrolledWindow::builder()
                .hscrollbar_policy(gtk::PolicyType::Never)
                .min_content_width(400)
                .child(&fbox)
                .build();

            window.set_child(Some(&scrolled));
            window.present();
        });

        app.run()
    }

    /*
    fn create_thumbnail(&self, img_path: &PathBuf) -> Image {
        Image::from_file(img_path)
    }
    */
}
