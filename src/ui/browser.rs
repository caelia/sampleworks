#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, FlowBox, Image, Picture};

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use rodio::{Decoder, OutputStream, source::Source};

use std::collections::HashMap;

const CSS: &str = "
    picture {
        border: 1px solid #c0c0c0;
    }
";

pub struct DumbBrowser {
    snd_dir: PathBuf,
    img_dir: PathBuf,
}

impl DumbBrowser {
    pub fn new(snd_dir: PathBuf, img_dir: PathBuf) -> DumbBrowser {
        DumbBrowser { snd_dir, img_dir }
    }

    pub fn run(&self) -> glib::ExitCode {
    // pub fn run(&self) -> glib::signal::SignalHandlerId {
        let app = Application::builder()
            .application_id("org.sampleworks.SWPrototype")
            .build();

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
                .min_children_per_line(2)
                .selection_mode(gtk::SelectionMode::None)
                .build();

            for file in &img_files {
                match file {
                    Some(path) => {
                        let img = Picture::for_filename(&path);
                        let ectrl = gtk::GestureClick::new();
                        let message = format!("{:?}", path);
                        ectrl.connect_pressed(move |_, _, _, _| {
                            println!("{}", message)
                        });
                        img.add_controller(ectrl);
                        fbox.insert(&img, -1);
                    },
                    None => (),
                }
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
