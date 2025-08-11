#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use rodio::{Decoder, decoder::DecoderError, source::Source};
// use sndfile::{OpenOptions, ReadOptions};
use anyhow::Result;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::sync::{mpsc, Sender, Receiver};
use std::thread::sleep;
use std::time::Duration;
use crate::messaging::{ACReq, ACRsp};

pub enum QItem {
    File(String, PathBuf),
    Data(String, Vec<f32>),
    Loop,
    End,
}

pub enum AudioState {
    Stopped,
    Paused,
    Running,
}

pub type Queue = Vec<QItem>;

pub struct Controller {
    req_rx: Receiver<ACRequest>,
    rsp_tx: Sender<Result<()>>,
    state: AudioState,
    queue: Option<Queue>,
}

impl Controller {
    pub fn new(req_rx: Receiver<ACRequest>, rsp_tx: Sender<Result<()>>) -> Self {
        Controller {
            req_rx,
            rsp_tx,
            state: AudioState::Stopped,
            queue: None,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let interval = Duration::from_millis(10);
        loop {
            match self.req_rx.try_recv() {
                Ok(ACReq::Audition) => (),
                Ok(ACReq::Stop) => (),
                _ => (),
            }
            sleep(interval);
        }
    }

    pub fn audition(&self) -> Result<()> {
        Ok(())        
    }

    pub fn play_queue(&self, queue: Queue, do_loop: bool) -> Result<()> {
        Ok(())
    }
}

pub fn stream_data(path: &PathBuf) -> Vec<f32> {
    let file = File::open(path).unwrap();
    let dec = Decoder::new(file).unwrap();
    dec.collect::<Vec<f32>>()
}

pub fn get_min_maxes(data: Vec<f32>, nframes: usize, width: usize) -> (f32, f32, Vec<(f32, f32)>) {
    let mut min_maxes = Vec::new();
    let mut all_max: f32 = f32::MIN + 2.0;
    let mut all_min: f32 = f32::MAX - 1.0;
    // let mut all_min: f32 = f32::MAX;
    let group_size = nframes / width;
    for i in 0..width {
        let mut range_max: f32 = f32::MIN + 2.0;
        let mut range_min: f32 = f32::MAX - 1.0;
        // let mut range_min: f32 = f32::MAX;
        for j in 0..group_size {
            let idx = i * group_size + j;
            if idx > nframes - 1 {
                break
            }
            let mut smp = data[idx];
            if smp < (f32::MIN + 2.0) {
                smp = f32::MIN + 2.0;
            } else if smp > f32::MAX {
                smp = f32::MAX;
            }
            if smp > range_max {
                range_max = smp;
            }
            if smp < range_min {
                range_min = smp;
            }
            if smp > all_max {
                all_max = smp;
            }
            if smp < all_min {
                all_min = smp;
            }
        }
        min_maxes.push((range_min, range_max))
    }

    (all_min, all_max, min_maxes)
}

pub fn dump(path: PathBuf, width: usize) {
    let data = stream_data(&path);
    let length = data.len();
    let (all_min, all_max, minmaxes) = get_min_maxes(data, length, width);
    println!(":: {:?} ::::::::", path);
    println!("   COUNT: {}", length);
    for i in 0..width {
        let (min, max) = minmaxes[i];
        println!("   {:>7}: {} / {}", i, min, max);
    }
    println!("   MIN: {} / MAX: {}", all_min, all_max);
    println!();
}

pub fn dump_raw(path: PathBuf) {
    let data = stream_data(&path);
    println!(":: {:?} ::::::::", path);
    println!("   COUNT: {}", data.len());
    for datum in data {
        println!("   {}", datum);
    }
    println!();
}
