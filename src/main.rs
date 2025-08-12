#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

/*
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

use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
// use sndfile::{OpenOptions, ReadOptions};
use rodio::{Decoder, decoder::DecoderError, source::Source}; 
use image::Rgb;
use anyhow::Result;
use configura;

use crate::audio::*;
use crate::img::*;
use crate::project::{Project, SourceSpec};
use crate::ui::browser::DumbBrowser;
use crate::config::SWConfig;

const DEMO: bool = true;


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
                (200, 80),
                Fill::Gradient(Rgb([0, 0, 248]), Rgb([248, 0, 0]), 16, 60),
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

    let browser = DumbBrowser::new(
        src_path,
        proj_path.join("images")
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

*/

// THE FOLLOWING CODE IS FOR toy-implementation BRANCH ONLY!

mod toy_audio;
use toy_audio::{Controller, Req, Rsp};

use anyhow::{anyhow, Result, Error};

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
    let mut controller = Controller::new(req_rx, rsp_tx);
    thread::spawn(move || {
        let _  = controller.run();
    });
    let thyme = Instant::now();
    req_tx.send(Req::Play(PathBuf::from(PATH1)))?;
    loop {
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
