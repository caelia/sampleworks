#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod audio;
mod img;

use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
// use sndfile::{OpenOptions, ReadOptions};
use rodio::{Decoder, decoder::DecoderError, source::Source}; 
use image::Rgb;
use crate::audio::*;
use crate::img::*;


fn main() {
    let args = std::env::args();
    if args.len() < 2 {
        println!("Please provide one or more file names.");
    } else {
        for fname in args.skip(1) {
            // dump(fname, 320);
            // dump_raw(fname);
            let img_width: u32 = 320;
            let img_height: u32 = 120;
            let raw_data = stream_data(&fname);
            let nframes = raw_data.len();
            let data = get_min_maxes(raw_data, nframes, img_width as usize);
            let (all_min, all_max, minmaxes) = data.clone();
            let vscale = (img_height as f32 / 2.0) / f32::max(i16::abs(all_min) as f32, i16::abs(all_max) as f32);
            // let mut wf_img = WaveformImg::new(img_width, img_height, vscale, Fill::Solid(Rgb([187, 0, 0])), Rgb([204, 204, 204]));
            let mut wf_img = WaveformImg::new(
                img_width,
                img_height,
                vscale,
                Fill::Gradient(Rgb([0, 0, 200]), Rgb([200, 0, 0]), 16, 96),
                Rgb([204, 204, 204]));
            wf_img.draw(minmaxes);
            let path = PathBuf::from(fname).with_extension("png");
            wf_img.save(&path);
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
