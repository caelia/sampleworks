pub mod browser;
pub mod wavethumb;

#[derive(Debug, Clone)]
pub enum Message {
    Play(String),
    Stop,
    Toggle(String),
}
