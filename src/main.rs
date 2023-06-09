#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod audio;
mod img;
mod project;
mod ui;
mod app_data;
mod util;

use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
// use sndfile::{OpenOptions, ReadOptions};
use rodio::{Decoder, decoder::DecoderError, source::Source}; 
use image::Rgb;
use anyhow::Result;

use crate::audio::*;
use crate::img::*;
use crate::project::{Project, SourceSpec};
use crate::ui::browser::DumbBrowser;


const SRC_PATH: &str = "/tmp/swtest-src";
const PROJ_PATH: &str = "/tmp/swtest-proj";


fn create_thumbs(src_path: PathBuf, proj_path: PathBuf) -> Result<()> {
    let project = Project::new(
        SourceSpec::Dir(src_path),
        proj_path,
    );
    let proj_result = project.init(true);
    // for fname in args.skip(1) {

    let files = project.get_src_files();

    match files {
        Ok(fls) => {
            project.create_thumbs(
                fls,
                (200, 80),
                Fill::Gradient(Rgb([0, 0, 248]), Rgb([248, 0, 0]), 16, 60),
                Rgb([221, 221, 221])
            )
        },
        Err(e) => Err(e.into()),
    }
}

fn main() -> Result<()> {
    let src_path = PathBuf::from(SRC_PATH);
    let proj_path = PathBuf::from(PROJ_PATH);

    let _ = create_thumbs(src_path.clone(), proj_path.clone());

    let browser = DumbBrowser::new(
        src_path,
        proj_path.join("images")
    );
    let _ = browser.run();
    Ok(())
}

    /*
    let file = BufReader::new(File::open(fname).unwrap());

    let (amn, amx, mm) = get_min_maxes(file, len);

    let vscale = (HEIGHT / 2) as f32 / (std::cmp::max(i16::abs(amn), i16::abs(amx))) as f32;
    
    let img = create_image(mm, vscale);
    img.save(Path::new(fname).with_extension("png"));
}
    */
