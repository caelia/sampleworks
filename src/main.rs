#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use sndfile::{OpenOptions, ReadOptions};
use rodio::{Decoder, decoder::DecoderError, source::Source}; 
use image::{ImageBuffer, Rgb, RgbImage, Pixel};

const WIDTH: u32 = 320;
const HEIGHT: u32 = 200;
const BG_COLOR: [u8;3] = [224, 224, 224];
const FG_COLOR: [u8;3] = [127, 0, 159];

fn n_frames(path: &PathBuf) -> u64 {
    let mut sf = OpenOptions::ReadOnly(ReadOptions::Auto).from_path(path).unwrap();
    sf.len().unwrap()
}

fn get_min_maxes(reader: BufReader<File>, nframes: u64) -> (i16, i16, Vec<(i16, i16)>) {
    let mut min_maxes = Vec::new();
    let mut dec = Decoder::new(reader).unwrap();

    let mut all_max: i16 = 0;
    let mut all_min: i16 = 0;
    'outer: for _ in 0..WIDTH {
        let mut range_max: i16 = 0;
        let mut range_min: i16 = 0;
        for _ in 0..(nframes / WIDTH as u64) {
            let smp = match dec.next() {
                Some(s) => s,
                None => break 'outer
            };
            if smp > range_max {
                range_max = smp;
            }
            if smp < range_min {
                range_min = smp;
            }
            if smp > all_max {
                all_max = smp;
            }
            if smp < all_min {
                all_min = smp;
            }
        }
        min_maxes.push((range_min, range_max))
    }

    (all_min, all_max, min_maxes)
}

fn scale_y(y0: i16, vscale: f32) -> u32 {
    let fheight = HEIGHT as f32;
    let fmid = (HEIGHT / 2) as f32;
    let fy = y0 as f32;
    (fmid + fy * vscale) as u32
}

fn draw_vline(ibuf: &mut RgbImage, color: Rgb<u8>, x: u32, y0: u32, y1_: u32) {
    let y1 = if y1_ >= HEIGHT {
        println!("WARNING: invalid y-coordinate: {}", y1_);
        HEIGHT - 1
    } else {
        y1_
    };
    for y in y0..=y1 {
        ibuf.put_pixel(x, y.into(), color);
    }
}

fn create_image(data: Vec<(i16, i16)>, vscale: f32) -> RgbImage {
    let mut img = RgbImage::new(WIDTH, HEIGHT);
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            img.put_pixel(x.into(), y.into(), Rgb(BG_COLOR));
        }
    }
    for i in 0..data.len() {
        let (y0, y1) = data[i];
        draw_vline(&mut img, Rgb(FG_COLOR), i as u32, scale_y(y0, vscale), scale_y(y1, vscale));
    }
    img
}

fn main() {
    let fname = "X2U_120_03_Junk_Foley_1.flac";
    // let fname = "i-know02.flac";
    let len = n_frames(&(PathBuf::from(fname)));

    let file = BufReader::new(File::open(fname).unwrap());

    let (amn, amx, mm) = get_min_maxes(file, len);

    let vscale = (HEIGHT / 2) as f32 / (std::cmp::max(i16::abs(amn), i16::abs(amx))) as f32;
    
    let img = create_image(mm, vscale);
    img.save(Path::new(fname).with_extension("png"));
}
