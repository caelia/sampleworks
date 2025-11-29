#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

pub mod browser;
pub mod wavethumb;

#[derive(Debug, Clone)]
pub enum Message {
    Play(String),
    Stop,
    Toggle(String),
}
