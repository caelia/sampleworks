#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod audio;
mod img;
mod project;

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


fn main() -> Result<()> {
    let project = Project::new(
        SourceSpec::Dir(PathBuf::from("/tmp/swtest-src")),
        PathBuf::from("/tmp/swtest-proj"),
    );
    let proj_result = project.init(true);
    // for fname in args.skip(1) {

    let mut files = vec![];
    match project.source {
        SourceSpec::Files(ref files_) => files = files_.to_vec(),
        SourceSpec::Dir(ref dir) => {
            let read_result = dir.read_dir();
            match read_result {
                Ok(entries) => {
                    for entry in entries {
                        match entry {
                            Ok(ent) => {
                                files.push(dir.join(ent.file_name()));
                            },
                            Err(e) => return Err(e.into())
                        }
                    }
                },
                Err(e) => return Err(e.into()),
            }
        }
    }

    for fname in files {
        // dump(fname, 320);
        // dump_raw(fname);
        let img_width: u32 = 320;
        let img_height: u32 = 120;
        let raw_data = stream_data(&fname);
        let nframes = raw_data.len();
        let data = get_min_maxes(raw_data, nframes, img_width as usize);
        let (all_min, all_max, minmaxes) = data.clone();
        let vscale = (img_height as f32 / 2.0) / f32::max(i16::abs(all_min) as f32, i16::abs(all_max) as f32);
        /*
        let half_height = img_height as f32 / 2.0;
        let abs_min = i16::abs(all_min);
        let abs_max = i16::abs(all_max);
        let max_data_height_ = i16::max(abs_min, abs_max);
        let max_data_height = max_data_height_ as f32;
        let vscale = half_height / max_data_height;
        */

        // let mut wf_img = WaveformImg::new(img_width, img_height, vscale, Fill::Solid(Rgb([187, 0, 0])), Rgb([204, 204, 204]));
        let mut wf_img = WaveformImg::new(
            img_width,
            img_height,
            vscale,
            Fill::Gradient(Rgb([0, 0, 200]), Rgb([200, 0, 0]), 16, 96),
            Rgb([204, 204, 204]));
        wf_img.draw(minmaxes);
        let path = project.get_image_path(&fname);
        println!("Saving image to {:?}", path);
        wf_img.save(&path);
    }

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
