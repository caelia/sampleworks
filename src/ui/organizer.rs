#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use iced::widget::canvas;
use iced::widget::canvas::{Program, Geometry};
use iced::{Element, Result, Task, Renderer, Rectangle, Theme};
use iced::mouse;

use super::Message;

pub struct Organizer {}

pub struct OrgItem {}

impl Organizer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self) -> Element<'_, Message> {
        canvas(OrgItem {})
            .into()
    }
}

impl <Message> Program<Message> for OrgItem {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        _renderer: &Renderer,
        _theme: &Theme,
        _bounds: Rectangle,
        _cursor: mouse::Cursor
    ) -> Vec<Geometry> {
        vec![]
    }
}
