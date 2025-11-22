#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod common;
mod config;
mod audio;
mod img;
mod project;
mod ui;
mod app_data;
mod caching;
mod util;
mod messaging;

// use sndfile::{OpenOptions, ReadOptions};
use rodio::{Decoder, decoder::DecoderError, source::Source}; 
use image::Rgb;
use anyhow::{Result, Error, anyhow};
use configura;

use iced::{Task, application};

use std::fs::File;
use std::io::BufReader;
use std::rc::Rc;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{Sender, Receiver, channel};
use std::thread;
use std::collections::HashMap;

use crate::audio::*;
use crate::img::*;
use crate::project::{Project, SourceSpec};
use crate::ui::browser::SampleBrowser;
use crate::config::SWConfig;
use crate::messaging::{ACReq, ACRsp, TxWrapper, RxWrapper};

///////////////////////////////////////////////////////////////////////
/// TEMPORARY STUFF ///////////////////////////////////////////////////
const DEMO: bool = true;

///////////////////////////////////////////////////////////////////////

fn create_thumbs(src_path: PathBuf, proj_path: PathBuf) -> Result<Vec<(PathBuf, PathBuf)>> {
    let project = Project::new(
        SourceSpec::Dir(Box::new(src_path)),
        proj_path,
    );
    // let proj_result = project.init(true);
    // for fname in args.skip(1) {

    let files = project.get_src_files();

    match files {
        Ok(fls) => {
            let file_map = project.get_thumbs(
                fls,
                (800, 320),
                Fill::Gradient(Rgb([0, 203, 0]), Rgb([18, 0, 170]), 64, 240),
                Rgb([221, 221, 221])
            ).unwrap();
            Ok(file_map)
        },
        Err(e) => Err(e.into()),
    }
}

// fn main() -> anyhow::Result<()> {
fn main() -> iced::Result {
    let cfg = match configura::load_config::<SWConfig>() {
        Ok(config) => config,
        Err(_) => SWConfig::default()
    };
    let src_path = &cfg.demo_source_path;
    let proj_path = &cfg.default_project_path.join("demo");
    if !proj_path.exists() {
        match std::fs::create_dir_all(&proj_path) {
            Ok(_) => (),
            Err(_) => panic!("failed to create project path")
        }
    }

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

    /*
    let browser = SampleBrowser::new(
        &project, TxWrapper::new(req_tx), RxWrapper::new(rsp_rx)
    );
    */
    application("SampleWorks", SampleBrowser::update, SampleBrowser::view)
        .run_with(move || (
            SampleBrowser::new(
                Rc::new(project.clone()), TxWrapper::new(req_tx), RxWrapper::new(rsp_rx)
            ),
            Task::none()
        ))
}
    /*
    match create_thumbs(src_path.clone(), proj_path.clone()) {
        Ok(file_map) => {
            let browser = SampleBrowser::new(
                &project,
                file_map,
                TxWrapper::new(req_tx),
                RxWrapper::new(rsp_rx)
            );
            // let _ = browser.run(file_map);
            Ok(())
        },
        Err(e) => Err(anyhow!(e))
    }
    */
