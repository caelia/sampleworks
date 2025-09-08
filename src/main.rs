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
use anyhow::Result;
use configura;

use std::fs::File;
use std::io::BufReader;
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

fn create_thumbs(src_path: PathBuf, proj_path: PathBuf) -> Result<()> {
    let project = Project::new(
        SourceSpec::Dir(src_path),
        proj_path,
    );
    let proj_result = project.init(true);
    // for fname in args.skip(1) {

    let files = project.get_src_files();

    match files {
        Ok(fls) => {
            project.create_thumbs(
                fls,
                // (200, 80),
                (800, 320),
                // Fill::Gradient(Rgb([0, 0, 248]), Rgb([248, 0, 0]), 16, 60),
                // Fill::Gradient(Rgb([0, 203, 0]), Rgb([18, 0, 170]), 16, 60),
                Fill::Gradient(Rgb([0, 203, 0]), Rgb([18, 0, 170]), 64, 240),
                Rgb([221, 221, 221])
            )
        },
        Err(e) => Err(e.into()),
    }
}

fn main() -> Result<()> {
    let cfg = match configura::load_config::<SWConfig>() {
        Ok(config) => config,
        Err(_) => SWConfig::default()
    };

    let src_path = cfg.demo_source_path;
    let proj_path = cfg.default_project_path.join("demo");
    if !proj_path.exists() {
        match std::fs::create_dir_all(&proj_path) {
            Ok(_) => (),
            Err(_) => panic!("failed to create project path")
        }
    }

    let _ = create_thumbs(src_path.clone(), proj_path.clone());

    let (req_tx, req_rx) = channel();
    let (rsp_tx, rsp_rx) = channel();

    let mut audio_controller = Controller::new(rsp_tx, req_rx);
    thread::spawn(move || {
        let _ = audio_controller.run(true);
    });
        
    let browser = SampleBrowser::new(
        src_path,
        proj_path.join("images"),
        TxWrapper::new(req_tx),
        RxWrapper::new(rsp_rx)
    );
    let _ = browser.run();
    Ok(())
}

    /*
    let file = BufReader::new(File::open(fname).unwrap());

    let (amn, amx, mm) = get_min_maxes(file, len);

    let vscale = (HEIGHT / 2) as f32 / (std::cmp::max(i16::abs(amn), i16::abs(amx))) as f32;
    
    let img = create_image(mm, vscale);
    img.save(Path::new(fname).with_extension("png"));
}
    */

/*
// THE FOLLOWING CODE IS FOR toy-implementation BRANCH ONLY!

mod toy_audio;
use toy_audio::{Controller, Req, Rsp};

use anyhow::{anyhow, Result, Error};
// use rand::prelude::*;

use std::thread;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::path::PathBuf;
use std::time::{Duration, Instant};

const PATH1: &str = "/+/music/sample-sets/ARCANE_PERCUSSION/KVE_WAV_LOOPS/KVE_BERIMBAU_PERC_LPS/KVE_085_Berimbau_Perc_14.flac";
const PATH2: &str = "/+/music/sample-sets/ARCANE_PERCUSSION/KVE_WAV_LOOPS/KVE_D_TABLA_PERC_LPS/KVE_150_D_Tabla_Perc_11.flac";
const PATH3: &str = "/+/music/sample-sets/ARCANE_PERCUSSION/KVE_WAV_LOOPS/KVE_SPACE_DRUM_LPS/KVE_096_Space_Drum_Perc_08.flac";

fn main() -> Result<()> {
    let (req_tx, req_rx) = channel();
    let (rsp_tx, rsp_rx) = channel();
    let mut controller = Controller::new(req_rx, rsp_tx, 1.0);
    thread::spawn(move || {
        let _  = controller.run(true);
    });
    /*
    let mut rng = rand::rng();
    let nums: Vec<u16> = (1..10000).collect();
    let small_sleep = Duration::from_secs(1);
    */
    let thyme = Instant::now();
    let paths = vec![PathBuf::from(PATH1), PathBuf::from(PATH2), PathBuf::from(PATH3)];
    req_tx.send(Req::Play(paths))?;
    loop {
        /*
        let x = nums.choose(&mut rng).unwrap();
        if *x > 9998 {
            let _ = req_tx.send(Req::Pause);
            thread::sleep(small_sleep);
            let _ = req_tx.send(Req::Resume);
        }
        */
        match rsp_rx.try_recv() {
            Ok(Rsp::Running) => (),
            Ok(Rsp::Paused(_)) => (),
            Ok(Rsp::Done) => {
                println!("OK!");
                break;
            },
            Ok(Rsp::Error(pos)) => {
                println!("Audio error at position {}.", pos);
                break;
            },
            Err(_) => (),
            /*
            Err(_) => {
                println!("Failed to read response.");
                break;
            }
            */
        }
    }
    let elapsed_time = thyme.elapsed().as_secs_f32();
    println!("Time: {} seconds.", elapsed_time);
    Ok(())
}
*/
