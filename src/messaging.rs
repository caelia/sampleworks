#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use anyhow::{Result, Error, anyhow};

use std::path::PathBuf;
use std::sync::mpsc::{Sender, SyncSender, Receiver, channel, sync_channel};
use std::time::Duration;
use std::cell::RefCell;
use std::thread::sleep;

/*
pub enum Message {
    FatalError(anyhow::Error),
    Error(anyhow::Error),
    Warning(String),
    Info(String),
    Debug(String),
}

pub trait Messenger {
    fn send(msg: Message) -> Result<()>;
}
*/

#[derive(Debug, Clone)]
pub enum Position {
    Start,
    End,
    TimePoint(f32),
    PercentagePoint(f32)
}

#[derive(Debug, Clone)]
pub enum ACReq {
    Audition(PathBuf),
    QueueAdd(PathBuf),
    QueueInsert(PathBuf,usize),
    QueueRemove(usize),
    PlayQueue,
    Pause,
    Resume,
    Stop,
    GoTo(Position),
}

#[derive(Debug, Clone)]
pub enum ACRsp {
    Running,
    Paused(f32),
    Done,
    Error(f32),
}

const MAX_RETRIES: usize = 5;
const RETRY_INTERVAL: Duration = Duration::from_millis(1);

pub struct RxWrapper<Msg> {
    rx: RefCell<Receiver<Msg>>,
}

impl<Msg> RxWrapper<Msg> {
    pub fn new(rx: Receiver<Msg>) -> Self {
        RxWrapper { rx: RefCell::new(rx) }
    }

    pub fn try_recv(&self) -> Result<Msg> {
        match self.rx.try_borrow() {
            Ok(rx) => {
                let mut attempts: usize = 0;
                loop {
                    match rx.try_recv() {
                        Ok(rsp) => return Ok(rsp),
                        Err(e) if attempts >= MAX_RETRIES => {
                            return Err(anyhow!(e));
                        },
                        Err(_) => (),
                    }
                    attempts += 1;
                    sleep(RETRY_INTERVAL);
                }
            },
            Err(e) => return Err(anyhow!(e)),
        };
    }
}

pub struct TxWrapper<Msg> {
    tx: RefCell<Sender<Msg>>,
}

impl<Msg: Clone> TxWrapper<Msg> {
    pub fn new(tx: Sender<Msg>) -> Self {
        TxWrapper { tx: RefCell::new(tx) }
    }

    pub fn send(&self, msg: Msg) -> Result<()> {
        match self.tx.try_borrow() {
            Ok(tx) => {
                let mut attempts: usize = 0;
                loop {
                    match tx.send(msg.clone()) {
                        Ok(()) => return Ok(()),
                        Err(e) if attempts >= MAX_RETRIES => {
                            return Err(anyhow!("Failed to send message."));
                        },
                        Err(_) => (),
                    }
                    attempts += 1;
                    sleep(RETRY_INTERVAL);
                }
            },
            Err(e) => return Err(anyhow!(e)),
        }
    }
}
