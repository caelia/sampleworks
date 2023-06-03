#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod audio;

use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
// use sndfile::{OpenOptions, ReadOptions};
use rodio::{Decoder, decoder::DecoderError, source::Source}; 
use image::{ImageBuffer, Rgb, RgbImage, Pixel};
use crate::audio::*;


fn main() {
    let args = std::env::args();
    if args.len() < 2 {
        println!("Please provide one or more file names.");
    } else {
        for fname in args.skip(1) {
            // dump(fname, 320);
            dump_raw(fname);
        }
    }

    /*
    let file = BufReader::new(File::open(fname).unwrap());

    let (amn, amx, mm) = get_min_maxes(file, len);

    let vscale = (HEIGHT / 2) as f32 / (std::cmp::max(i16::abs(amn), i16::abs(amx))) as f32;
    
    let img = create_image(mm, vscale);
    img.save(Path::new(fname).with_extension("png"));
    */
}
