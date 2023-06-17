use directories::ProjectDirs;
use toml::Table;
use blake3::{Hash, Hasher};
use rusqlite::{Connection, Error::QueryReturnedNoRows};
use anyhow::{anyhow, Result};

use std::collections::HashMap;
use std::path::{Path, PathBuf};

fn base_dirs() -> Result<&'static ProjectDirs> {
    match ProjectDirs::from("org", "sampleworks", "Sampleworks") {
        Some(pd) => Ok(&pd),
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

pub struct SourceIndex {
    conn: Connection,
}

impl SourceIndex {
    pub fn new() -> Result<Self> {
        let data_dir = match base_dirs() {
            Ok(ref bd) => bd.data_local_dir(),
            Err(e) => return Err(anyhow!(e)),
        };
        let db_path = data_dir.join("source_index.db");
        let conn = Connection::open(db_path)?;
        Ok(SourceIndex { conn })
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