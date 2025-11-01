mod imp;

use gtk4 as gtk;
use gtk::glib;
use glib::Object;

glib::wrapper! {
    pub struct Thumbnail(ObjectSubclass<imp::Thumbnail>)
        @extends gtk::Frame, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl Thumbnail {
    pub fn new() -> Self {
        Object::builder().build()
    }
}
