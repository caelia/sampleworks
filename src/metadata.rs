#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

pub mod db;
mod sql;

use std::path::PathBuf;

pub enum PType {
    Tag,
    String,
    Path,
    Bool,
    Date,
    Timestamp,
    Int,
    UInt,
    Float,
    // Custom(String),
}

pub struct Property {
    pub name: String,
    pub ptype: PType,
    pub value: String   
}
