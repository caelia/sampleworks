#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
// use sndfile::{OpenOptions, ReadOptions};
use rodio::{Decoder, decoder::DecoderError, source::Source}; 


pub fn stream_data(path: &String) -> Vec<i16> {
    let reader = BufReader::new(File::open(path).unwrap());
    let dec = Decoder::new(reader).unwrap();
    dec.collect::<Vec<i16>>()
}

pub fn get_min_maxes(data: Vec<i16>, nframes: usize, width: usize) -> (i16, i16, Vec<(i16, i16)>) {
    let mut min_maxes = Vec::new();
    let mut all_max: i16 = i16::MIN;
    let mut all_min: i16 = i16::MAX;
    let group_size = nframes / width;
    for i in 0..width {
        let mut range_max: i16 = i16::MIN;
        let mut range_min: i16 = i16::MAX;
        for j in 0..group_size {
            let idx = i * group_size + j;
            if idx > nframes - 1 {
                break
            }
            let smp = data[idx];
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

pub fn dump(path: String, width: usize) {
    let data = stream_data(&path);
    let length = data.len();
    let (all_min, all_max, minmaxes) = get_min_maxes(data, length, width);
    println!(":: {} ::::::::", path);
    println!("   COUNT: {}", length);
    for i in 0..width {
        let (min, max) = minmaxes[i];
        println!("   {:>7}: {} / {}", i, min, max);
    }
    println!("   MIN: {} / MAX: {}", all_min, all_max);
    println!();
}

pub fn dump_raw(path: String) {
    let data = stream_data(&path);
    println!(":: {} ::::::::", path);
    println!("   COUNT: {}", data.len());
    for datum in data {
        println!("   {}", datum);
    }
    println!();
}
