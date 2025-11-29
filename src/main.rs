#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod common;
mod config;
mod audio;
mod img;
mod sound_object;
mod project;
mod ui;
mod util;
mod messaging;

// use sndfile::{OpenOptions, ReadOptions};
use image::Rgb;
use anyhow::Result;
use configura;

use iced::{Task, application};

use std::rc::Rc;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::thread;

use crate::audio::*;
use crate::img::*;
use crate::project::{Project, SourceSpec};
use crate::ui::browser::SampleBrowser;
use crate::config::SWConfig;
use crate::messaging::{TxWrapper, RxWrapper};

///////////////////////////////////////////////////////////////////////
/// TEMPORARY STUFF ///////////////////////////////////////////////////
// const DEMO: bool = true;

fn demo_msg(src_path: &PathBuf, proj_path: &PathBuf) {
   println!( 
 r#"
======================================================================  
|                                                                    |
|    Welcome to the SampleWorks demo!                                |
|    --------------------------------                                |
|                                                                    |
|    A collection of sound samples is being installed in             |
|    {:?}.   |
|                                                                    |
|    You may also place your own samples in this directory, and      |
|    they will be detected and displayed by SampleWorks.             |
|                                                                    |
|    Thumbnail images will be generated in                           |
|    {:?}.   |
|                                                                    |
|    If you like any of the provided samples and wish to use them    |
|    in your own projects, please see 'demo_samples/LICENSE.txt'     |
|    for licensing info.                                             |
|                                                                    |
======================================================================
"#,
    src_path,
    proj_path
    );
}

///////////////////////////////////////////////////////////////////////

fn main() -> iced::Result {
    let cfg = match configura::load_config::<SWConfig>() {
        Ok(config) => config,
        Err(_) => SWConfig::default()
    };
    let src_path = &cfg.demo_source_path;
    if !src_path.exists() {
        match std::fs::create_dir_all(&src_path) {
            Ok(_) => (),
            Err(_) => panic!("failed to create source path")
        }
    }
    let dir_iter = std::fs::read_dir(&cfg.demo_source_source)
        .expect("Oops! Can't read demo sample directory.");
    for entry in dir_iter {
        let entry = entry.expect("error reading dir entry");
        let src = entry.path();
        let fname = entry.file_name();
        let dest = src_path.join(&fname);
        std::fs::copy(src, dest)
            .expect(format!("failed to copy '{:?}'", fname).as_str());
    }
    let proj_path = &cfg.default_project_path.join("demo");
    if !proj_path.exists() {
        match std::fs::create_dir_all(&proj_path) {
            Ok(_) => (),
            Err(_) => panic!("failed to create project path")
        }
    }

    demo_msg(&src_path, &proj_path);

    let mut project = Project::new(
        SourceSpec::Dir(Box::new(src_path.clone())),
        proj_path.clone(),
    );

    let (req_tx, req_rx) = channel();
    let (rsp_tx, rsp_rx) = channel();

    let mut audio_controller = Controller::new(rsp_tx, req_rx);
    thread::spawn(move || {
        let _ = audio_controller.run(true);
    });
        
    let _ = project.load_objects();
    project.load_thumbs(
        (800, 320),
        Fill::Gradient(Rgb([0, 203, 0]), Rgb([18, 0, 170]), 64, 240),
        Rgb([221, 221, 221])
    );

    application("SampleWorks", SampleBrowser::update, SampleBrowser::view)
        .run_with(move || (
            SampleBrowser::new(
                Rc::new(project), TxWrapper::new(req_tx), RxWrapper::new(rsp_rx)
            ),
            Task::none()
        ))
}
