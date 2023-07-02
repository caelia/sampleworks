use directories::ProjectDirs;
use toml::Table;
use blake3::{Hash, Hasher};
use rusqlite::{Connection, Error::QueryReturnedNoRows};
use anyhow::{anyhow, Result};

use std::collections::HashMap;
use std::path::{Path, PathBuf};

fn base_dirs() -> Result<ProjectDirs> {
    match ProjectDirs::from("org", "sampleworks", "Sampleworks") {
        Some(pd) => Ok(pd.clone()),
        None => Err(anyhow!("No valid home directory found.")),
    }
}

const DB_INIT_SQL: &str = "
    CREATE TABLE sources_x_hashes (
        source TEXT UNIQUE NOT NULL,
        hash TEXT UNIQUE NOT NULL,
    );
    CREATE UNIQUE INDEX s on sources_x_hashes(source);
    CREATE UNIQUE INDEX h on sources_x_hashes(hash);
";
const SRC_2_HASH_SQL: &str = "
    SELECT hash FROM sources_x_hashes WHERE source = ?1;
";
const HASH_2_SRC_SQL: &str = "
    SELECT source FROM sources_x_hashes WHERE hash = ?1;
";

pub enum Domain {
    Source,
    Temp,
    Project(&'static Path),
}
pub struct Index {
    conn: Connection,
}

impl Index {
    pub fn new(domain: Domain) -> Result<Self> {
        let db_path = match domain {
            Domain::Source => {
                match base_dirs() {
                    Ok(ref bd) => {
                        let dir = bd.data_local_dir();
                        dir.join("source_index.db")
                    },
                    Err(e) => return Err(anyhow!(e)),
                }
            },
            Domain::Temp => {
                match base_dirs() {
                    Ok(ref bd) => {
                        let dir = match bd.runtime_dir() {
                            Some(rd) => rd,
                            None => bd.cache_dir(),
                        };
                        dir.join("temp_index.db")
                    },
                    Err(e) => return Err(anyhow!(e)),
                }
            },
            Domain::Project(path) => {
                path.join("project_index.db")
            }
        };
        let conn = Connection::open(db_path)?;
        Ok(Index { conn })
    }

    pub fn init(&mut self) -> Result<()> {
        match self.conn.execute(DB_INIT_SQL, []) {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow!(e)),
        }
    }

    pub fn add(&mut self, src: String) -> Result<()> {
        todo!()     
    }

    pub fn update(&mut self, src: String) -> Result<()> {
        todo!()
    }

    pub fn verify(&self, src: String) -> Result<bool> {
        todo!()     
    }

    pub fn delete(&mut self, src: String) -> Result<()> {
        todo!()
    }

    pub fn src2hash(&self, src: String) -> Result<Option<String>> {
        match self.conn.query_row(
            SRC_2_HASH_SQL,
            [src],
            |row| row.get(0),
        ) {
            Ok(data) => Ok(Some(data)),
            Err(QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(anyhow!(e)),
        }
    }

    pub fn hash2src(&self, hash: String) -> Result<Option<String>> {
        match self.conn.query_row(
            HASH_2_SRC_SQL,
            [hash],
            |row| row.get(0),
        ) {
            Ok(data) => Ok(Some(data)),
            Err(QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(anyhow!(e)),
        }
    }
}