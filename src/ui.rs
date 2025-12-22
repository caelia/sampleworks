pub mod browser;
pub mod organizer;
pub mod window;
pub mod wavethumb;

#[derive(Debug, Clone)]
pub enum Message {
    Play(String),
    Stop,
    Toggle(String),
    OnDrag(f32),
}
