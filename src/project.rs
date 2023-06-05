#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use anyhow::Result;

use std::path::{Path, PathBuf};
use std::fs::{create_dir, create_dir_all, remove_dir_all};
use std::collections::HashMap;

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
        // assert!(!self.proj_dir.exists());
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
}
