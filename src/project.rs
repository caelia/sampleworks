#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use anyhow::{Result, Error, anyhow};
use image::Rgb;
use blake3::hash as b3hash;

use std::path::{Path, PathBuf};
use std::fs::{create_dir, create_dir_all, remove_dir_all};
// use std::collections::{HashMap, BTreeSet};
use std::collections::{HashMap, BTreeMap};
use std::cmp::Ordering;

use crate::img::*;
use crate::audio::*;
use crate::common::SoundObject;


#[derive(Debug, Clone)]
pub enum SourceSpec {
    Dir(Box<PathBuf>),
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

pub fn get_image_path(proj_dir: PathBuf, hash_str: &String) -> (PathBuf, bool) {
    let image_dir = proj_dir.join("images");
    let path = image_dir.join(hash_str).with_extension("png");
    let exists = path.is_file();
    (path, exists)
}

#[derive(Debug, Clone)]
pub struct Project {
    pub source: SourceSpec,
    pub proj_dir: PathBuf,
    pub export_dirs: HashMap<String, PathBuf>,
    pub objects: BTreeMap<String, SoundObject>,
}

impl Project {
    pub fn new(source: SourceSpec, proj_dir: PathBuf) -> Self {
        source.validate_source();
        let export_dirs = HashMap::new();
        let objects = BTreeMap::new();
        Project {
            source,
            proj_dir,
            export_dirs,
            objects
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

    fn old_get_image_path(&self, sndfile_path: &PathBuf) -> PathBuf {
        let image_dir = self.proj_dir.join("images");
        let image_name_stem = sndfile_path
            .display()
            .to_string()
            .replace(std::path::MAIN_SEPARATOR_STR, "-");
        image_dir.join(PathBuf::from(image_name_stem).with_extension("png"))
    }

    pub fn get_image_path(&self, hash_str: &String) -> (PathBuf, bool) {
        let image_dir = self.proj_dir.join("images");
        let path = image_dir.join(hash_str).with_extension("png");
        let exists = path.is_file();
        (path, exists)
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

        files.sort_unstable();

        Ok(files)
    }

    pub fn get_thumbs(
            &self,
            files: Vec<PathBuf>,
            size: (u32, u32),
            fg: Fill,
            bg: Rgb<u8>) -> Result<Vec<(PathBuf, PathBuf)>> {
        let mut file_map = vec![];
        for fname in files {
            let hash = match std::fs::read(&fname) {
                Ok(bytes) => b3hash(bytes.as_slice()).to_string(),
                Err(e) => return Err(anyhow!(e)),
            };
            let (path, exists) = self.get_image_path(&hash);
            if !exists {
                let (width, height) = size;
                let raw_data = stream_data(&fname);
                let nframes = raw_data.len();

                let data = get_min_maxes(raw_data, nframes, width as usize);
                let (all_min, all_max, minmaxes) = data.clone();
                let vscale = (height as f32 / 2.0) / f32::max(f32::abs(all_min), f32::abs(all_max));

                // UH-UH! Need to get rid of fg.clone()
                let mut wf_img = WaveformImg::new(width, height, vscale, fg.clone(), bg);
                wf_img.draw(minmaxes);
                wf_img.save(&path);
            }
            file_map.push((path, fname));
        }
        
        Ok(file_map)
    }

    pub fn load_objects(&mut self) -> Result<()> {
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
        files.sort_unstable();
        
        for f in files {
            match SoundObject::new(f.clone()) {
                Ok(obj) => {
                    let id = f.file_name().unwrap().to_string_lossy().into_owned();
                    self.objects.insert(id.clone(), obj);
                },
                Err(e) => return Err(anyhow!(e)),
            }
        }
        Ok(())
    }

    pub fn load_thumbs(
            &mut self,
            size: (u32, u32),
            fg: Fill,
            bg: Rgb<u8>) {
        // for id in self.objects.keys() {
        // let objects = self.objects;
        for (id, obj) in self.objects.iter_mut() {
            // let (path, exists) = self.get_image_path(id);
            let (path, exists) = get_image_path(self.proj_dir.clone(), id);
            if !exists {
                let (width, height) = size;
                let raw_data = stream_data(&obj.content);
                let nframes = raw_data.len();

                let data = get_min_maxes(raw_data, nframes, width as usize);
                // let (all_min, all_max, minmaxes) = data.clone();
                let (all_min, all_max, minmaxes) = data;
                let vscale = (height as f32 / 2.0) / f32::max(f32::abs(all_min), f32::abs(all_max));

                // UH-UH! Need to get rid of fg.clone()
                let mut wf_img = WaveformImg::new(width, height, vscale, fg.clone(), bg);
                wf_img.draw(minmaxes);
                wf_img.save(&path);
            }
            match obj {
                SoundObject { thumbnail: Some(pth), .. } => (),
                o => {
                    // obj = SoundObject { content: o.content, thumbnail: Some(path)};
                    o.thumbnail = Some(path)
                }
            }
        }
    }

    pub fn get_object(&self, id: &String) -> Option<&SoundObject> {
        self.objects.get(id)
    }
}
