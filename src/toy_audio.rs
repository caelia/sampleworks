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
use std::iter::Iterator;

/*
pub enum QItem {
    Play(PathBuf),
    Pause(Duration),
    End,
    Loop,
}
*/

pub struct Queue {
    items: Vec<PathBuf>,
    idx: usize,
    looping: bool,
}

impl Queue {
    pub fn new(looping: bool) -> Self {
        Queue {
            items: vec![],
            idx: 0,
            looping,
        }
    }
}

impl Iterator for Queue {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        match self.items.get(self.idx) {
            Some(item) => {
                let result = Some(item.clone());
                self.idx += 1;
                result
            },
            None if self.looping && !self.items.is_empty() => {
                self.idx = 1;
                Some(self.items.get(0).unwrap().clone())
            },
            None => None
        }
    }
}

pub enum Req {
    Play(Vec<PathBuf>),
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
    gap: Duration,
    state: State,
}

impl Controller {
    pub fn new(
            from_main: Receiver<Req>,
            to_main: Sender<Rsp>,
            // from_worker: Receiver<Rsp>,
            // to_worker: Sender<Req>
            gap: f32,
            ) -> Self {
        Controller {
            from_main,
            to_main,
            // from_worker,
            // to_worker,
            gap: Duration::from_secs_f32(gap),
            state: State::Stopped,
        }
    }

    pub fn run(&mut self, looping: bool) -> Result<()> {
        let poll_interval = Duration::from_millis(10);
        let mut queue = Queue::new(looping);
        let output = OutputStreamBuilder::open_default_stream()
            .expect("Failed to open default audio stream.");
        let sink = Sink::connect_new(&output.mixer());
        loop {
            match self.from_main.try_recv() {
                Ok(Req::Play(paths)) => {
                    queue.items = paths;
                    match queue.next() {
                        Some(path) => {
                            let file = File::open(path)?;
                            let src = Decoder::try_from(file)?;
                            sink.append(src);
                        },
                        None => {
                            self.to_main.send(Rsp::Done)?;
                            break;
                        }
                    }
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
            if sink.empty() {
                match queue.next() {
                    Some(path) => {
                        sleep(self.gap);
                        let file = File::open(path)?;
                        let src = Decoder::try_from(file)?;
                        sink.append(src);
                    },
                    None => {
                        self.to_main.send(Rsp::Done)?;
                        break;
                    }
                }
            } else {
                sleep(poll_interval);
            }
        }
        Ok(())
    }
}
