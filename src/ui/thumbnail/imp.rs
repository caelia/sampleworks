use gtk4 as gtk;
use gtk::glib;
use gtk::subclass::prelude::*;

use std::path::PathBuf;

#[derive(Default)]
pub struct Thumbnail {
    snd_path: PathBuf,
    hash: String,
}

#[glib::object_subclass]
impl ObjectSubclass for Thumbnail {
    const NAME: &'static str = "SampleWorksThumbnail";
    type Type = super::Thumbnail;
    type ParentType = gtk::Frame;
}

impl ObjectImpl for Thumbnail {}

impl WidgetImpl for Thumbnail {}

impl FrameImpl for Thumbnail {}
