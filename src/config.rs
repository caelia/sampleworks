use configura::{Config, Format, formats::JsonFormat, load_config};
use directories::{UserDirs, ProjectDirs};
use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow};
use std::path::PathBuf;
use crate::common::AudioFormat;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct SWConfig {
    pub default_project_path: PathBuf,
    pub source_paths: Vec<PathBuf>,
    pub demo_source_path: PathBuf,
    pub demo_source_source: PathBuf,
    pub audio_cache_path: PathBuf,
    pub image_cache_path: PathBuf,
    pub default_cache_format: AudioFormat,
    pub default_export_format: AudioFormat
}

fn project_dirs() -> Result<ProjectDirs> {
    let pdirs = ProjectDirs::from("org", "sampleworks", "sampleworks");
    match pdirs {
        Some(dirs) => Ok(dirs),
        None => Err(anyhow!("couldn't initialize ProjectDirs struct"))
    }
}

impl Default for SWConfig {
    fn default() -> Self {
        let pdirs = project_dirs().expect("failed to get project dirs struct");
        let udirs = UserDirs::new().unwrap();
        let default_project_path = udirs.document_dir().unwrap().join("sampleworks");
        let demo_source_path = pdirs.data_dir().join("demo_samples");
        let demo_source_source = PathBuf::from("demo_samples");
        let source_paths = vec![demo_source_path.clone()];
        let audio_cache_path = pdirs.cache_dir().join("audio");
        let image_cache_path = pdirs.cache_dir().join("images");
        let default_cache_format = AudioFormat::FLAC;
        let default_export_format = AudioFormat::WAV;
        SWConfig {
            default_project_path,
            source_paths,
            demo_source_path,
            demo_source_source,
            audio_cache_path,
            image_cache_path,
            default_cache_format,
            default_export_format,
        }
    }
}

impl Config for SWConfig {
    type FormatType = JsonFormat;
    type FormatContext = ();

    fn config_path_and_filename(_home_dir: &std::path::Path) -> (Option<PathBuf>, &str) {
        let pdirs = project_dirs().unwrap();
        (Some(PathBuf::from(pdirs.config_dir())), "config")
    }
}
