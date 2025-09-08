#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use anyhow::{Result, Error, anyhow};
use image::Rgb;

use std::path::{Path, PathBuf};
use std::fs::{create_dir, create_dir_all, remove_dir_all};
use std::collections::{HashMap, BTreeSet};
use std::cmp::Ordering;

use crate::img::*;
use crate::audio::*;


pub enum SourceSpec {
    Dir(PathBuf),
    Files(Vec<PathBuf>),
}

impl SourceSpec {
    fn validate_source(&self) {
        match self {
            SourceSpec::Dir(path) => assert!(path.is_dir()),
            SourceSpec::Files(paths) => {
                assert!(&paths.iter().all(|path| {
                    path.is_file() && path.is_absolute()
                }))
            },
        }
    }
}

pub struct Project {
    pub source: SourceSpec,
    pub proj_dir: PathBuf,
    pub export_dirs: HashMap<String, PathBuf>,
}

impl Project {
    pub fn new(source: SourceSpec, proj_dir: PathBuf) -> Self {
        source.validate_source();
        let export_dirs = HashMap::new();
        Project {
            source,
            proj_dir,
            export_dirs,
        }
    }

    pub fn init(&self, overwrite: bool) -> Result<()> {
        if self.proj_dir.exists() {
            if overwrite {
                remove_dir_all(&self.proj_dir)?;
            } else {
                panic!("Project directory already exists.")
            }
        }
        create_dir_all(&self.proj_dir)?;
        create_dir(self.proj_dir.join("images"))?;
        Ok(())
    }

    pub fn get_image_path(&self, sndfile_path: &PathBuf) -> PathBuf {
        let image_dir = self.proj_dir.join("images");
        let image_name_stem = sndfile_path
            .display()
            .to_string()
            .replace(std::path::MAIN_SEPARATOR_STR, "-");
        image_dir.join(PathBuf::from(image_name_stem).with_extension("png"))
    }

    pub fn get_src_files(&self) -> Result<Vec<PathBuf>> {
        let mut files = vec![];
        match self.source {
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

        Ok(files)
    }

    pub fn create_thumbs(
            &self,
            files: Vec<PathBuf>,
            size: (u32, u32),
            fg: Fill,
            bg: Rgb<u8>) -> Result<Vec<(PathBuf, PathBuf)>> {
        let mut file_map = vec![];
        for fname in files {
            let (width, height) = size;
            let raw_data = stream_data(&fname);
            let nframes = raw_data.len();

            let data = get_min_maxes(raw_data, nframes, width as usize);
            let (all_min, all_max, minmaxes) = data.clone();
            let vscale = (height as f32 / 2.0) / f32::max(f32::abs(all_min), f32::abs(all_max));

            // UH-UH! Need to get rid of fg.clone()
            let mut wf_img = WaveformImg::new(width, height, vscale, fg.clone(), bg);
            wf_img.draw(minmaxes);
            let path = self.get_image_path(&fname);
            wf_img.save(&path);
            file_map.push((path, fname));
        }
        
        Ok(file_map)
    }
}
