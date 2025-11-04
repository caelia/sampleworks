#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use iced::advanced::{Widget, image::Image};

use std::path::PathBuf;
use std::rc::Rc;

use crate::messaging::{ACReq, ACRsp, TxWrapper, RxWrapper};

pub struct WaveThumb {
    file: PathBuf,
    snd_dir: PathBuf,
    img_dir: PathBuf,
    req_tx: Rc<TxWrapper<ACReq>>,
    rsp_rx: Rc<RxWrapper<ACRsp>>,
}

impl Widget for WaveThumb {
    fn size(&self) -> iced::Size {
        
    }

    fn layout(
        &self,
        tree: &mut iced::advanced::widget::Tree,
        renderer: &iced::Renderer,
        limits: &iced::advanced::layouts::Limits
    ) -> iced::advanced::layout::Node {
        
    }

    fn draw(
        &self,
        tree: &mut iced::advanced::widget::Tree,
        renderer: &iced::Renderer,
        theme: &iced::Theme,
        style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        
    }
}
