/*
use std::sync::mpsc::{Sender, SyncSender, Receiver, channel, sync_channel};
use anyhow::{self, Result};

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

pub enum Position {
    Start,
    End,
    TimePoint(f32),
    PercentagePoint(f32)
}

pub enum ACRequest {
    Audition,
    QueueAdd,
    QueueInsert,
    QueueRemove,
    PlayQueue,
    Pause,
    Resume,
    Stop,
    GoTo(Position),
}
