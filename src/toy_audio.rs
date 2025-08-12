#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use rodio::{Decoder, Sink, OutputStream, OutputStreamBuilder, source::Source};
use anyhow::{anyhow, Result, Error};
use std::sync::mpsc::{Sender, Receiver, channel};
use std::thread::{Scope, ScopedJoinHandle, Builder, sleep};
use std::time::Duration;
use std::path::PathBuf;
use std::fs::File;

pub enum Req {
    Play(PathBuf),
    Pause,
    Resume,
    Stop,
}

pub enum Rsp {
    Running,
    Paused(f32),
    Done,
    Error(f32),
}

enum State {
    Running,
    Paused,
    Stopped,
}

pub struct Controller {
    from_main: Receiver<Req>,
    to_main: Sender<Rsp>,
    // from_worker: Receiver<Rsp>,
    // to_worker: Sender<Req>,
    state: State,
}

impl Controller {
    pub fn new(
            from_main: Receiver<Req>,
            to_main: Sender<Rsp>,
            // from_worker: Receiver<Rsp>,
            // to_worker: Sender<Req>
            ) -> Self {
        Controller {
            from_main,
            to_main,
            // from_worker,
            // to_worker,
            state: State::Stopped,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let interval = Duration::from_millis(10);
        let output = OutputStreamBuilder::open_default_stream()
            .expect("Failed to open default audio stream.");
        let sink = Sink::connect_new(&output.mixer());
        loop {
            match self.from_main.try_recv() {
                Ok(Req::Play(path)) => {
                    let file = File::open(path).unwrap();
                    let src = Decoder::try_from(file).unwrap();
                    // output.mixer().add(src);
                    sink.append(src);
                    // sink.sleep_until_end();
                    // self.to_main.send(Rsp::Done)?;
                },
                Ok(Req::Pause) => {
                    sink.pause();
                },
                Ok(Req::Resume) => {
                    sink.play();
                },
                Ok(Req::Stop) => {
                    sink.stop();
                    self.to_main.send(Rsp::Done)?;
                    break;
                },
                Err(_) => {
                    ()
                    // println!(":: no message ::");
                    // break;
                }
            }
            sleep(interval);
            if sink.empty() {
                self.to_main.send(Rsp::Done)?;
                break;
            }
        }
        Ok(())
    }
}
