#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use rusqlite::{Connection, Error, params};
use anyhow::{anyhow, Result};

use std::path::PathBuf;

use super::sql;
use super::{PType, Property};

pub struct Db {
    conn: Connection,
}

impl Db {
    pub fn new(path: PathBuf) -> Result<Self>{
        let conn = Connection::open(&path)?;
        Ok(Db { conn })
    }

    pub fn setup(self) -> Result<()> {
        self.conn.execute("BEGIN;", sql::NULL_PARAMS)?;
        self.conn.execute(sql::CREATE_SYSTEM_TABLE, sql::NULL_PARAMS)?;
        self.conn.execute(sql::CREATE_PROPTYPE_TABLE, sql::NULL_PARAMS)?;
        self.conn.execute(sql::CREATE_VOCABULARY_TABLE, sql::NULL_PARAMS)?;
        self.conn.execute(sql::CREATE_OBJECT_PROPERTY_TABLE, sql::NULL_PARAMS)?;
        for q in sql::POPULATE_SYSTEM_TABLE_QQ {
            self.conn.execute(q, sql::NULL_PARAMS)?;
        }
        for q in sql::POPULATE_PROPTYPE_TABLE_QQ {
            self.conn.execute(q, sql::NULL_PARAMS)?;
        }
        self.conn.execute("COMMIT;", sql::NULL_PARAMS)?;
        Ok(())
    }

    pub fn set_object_property(&self, object_id: String, prop: Property) -> Result<()> {
        let (exists, compatible) = self.verify_property(&prop)?;
        if !compatible {
            return Err(anyhow!("Incompatible data types!"));
        }
        if !exists {
            self.add_property(&prop.name, prop.ptype);
        }
        self.conn.execute(sql::SET_OBJECT_PROPERTY, [object_id, prop.value, prop.name])?;
        Ok(())
    }

    fn verify_property(&self, prop: &Property) -> Result<(bool, bool)> {
        let mut stmt = self.conn.prepare(sql::VERIFY_PROPERTY)?;
        match stmt.query_one([prop.name], |row| {
            Ok((row.get(0)?, row.get(1)?))
        }) {
            Ok((_, type_name)) => {
                match (type_name, prop.ptype) {
                    ("tag", PType::Tag)
                    | ("string", PType::String)
                    | ("path", PType::Path)
                    | ("bool", PType::Bool)
                    | ("date", PType::Date)
                    | ("timestamp", PType::Timestamp)
                    | ("int", PType::Int)
                    | ("uint", PType::UInt)
                    | ("float", PType::Float) => Ok((true, true)),
                    (_, _) => Ok((true, false)),
                }
            },
            Err(Error::QueryReturnedNoRows) => Ok((false, true)),
            Err(e) => Err(anyhow!(e)),
        }
    }

    fn add_property(&self, name: &String, ptype: PType) -> Result<()> {
        let query = match ptype {
            PType::Tag => sql::ADD_TAG,
            PType::String => sql::ADD_STRING_PROPERTY,
            PType::Path => sql::ADD_PATH_PROPERTY,
            PType::Bool => sql::ADD_BOOLEAN_PROPERTY,
            PType::Date => sql::ADD_DATE_PROPERTY,
            PType::Timestamp => sql::ADD_TIMESTAMP_PROPERTY,
            PType::Int => sql::ADD_INT_PROPERTY,
            PType::UInt => sql::ADD_UINT_PROPERTY,
            PType::Float => sql::ADD_FLOAT_PROPERTY,
        };
        self.conn.execute(query, [name])?;
        Ok(())
    }
}

