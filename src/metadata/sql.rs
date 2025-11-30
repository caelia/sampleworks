#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

pub const NULL_PARAMS: &[&dyn rusqlite::types::ToSql] = &[];

pub const CREATE_SYSTEM_TABLE: &'static str =
"CREATE TABLE system (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    property TEXT UNIQUE NOT NULL,
    value TEXT NOT NULL      
);";

pub const CREATE_PROPTYPE_TABLE: &'static str =
"CREATE TABLE property_types (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT UNIQUE NOT NULL
);";

pub const CREATE_VOCABULARY_TABLE: &'static str =
"CREATE TABLE vocabulary (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    type INTEGER REFERENCES property_types(id)
);";

pub const CREATE_OBJECT_PROPERTY_TABLE: &'static str =
"CREATE TABLE object_properties (
    id INTEGER PRIMARY KEY AUTOINCREMENT,  
    object_id TEXT NOT NULL,
    property INTEGER REFERENCES vocabulary(id),
    value TEXT NOT NULL
);";

pub const POPULATE_SYSTEM_TABLE_QQ: [&'static str; 1] = [
    "INSERT INTO system (property, value) VALUES ('version', '1.0');"
];

pub const POPULATE_PROPTYPE_TABLE_QQ: [&'static str; 9] = [
"INSERT INTO property_types (name) VALUES ('tag');",
"INSERT INTO property_types (name) VALUES ('string');",
"INSERT INTO property_types (name) VALUES ('path');",
"INSERT INTO property_types (name) VALUES ('bool');",
"INSERT INTO property_types (name) VALUES ('date');",
"INSERT INTO property_types (name) VALUES ('timestamp');",
"INSERT INTO property_types (name) VALUES ('float');",
"INSERT INTO property_types (name) VALUES ('int');",
"INSERT INTO property_types (name) VALUES ('uint');",
];

pub const ADD_TAG: &'static str =
"INSERT INTO vocabulary (name, type)
    SELECT $1, id FROM property_types
    WHERE property_types.name = 'tag';";
pub const ADD_STRING_PROPERTY: &'static str =
"INSERT INTO vocabulary (name, type)
    SELECT $1, id FROM property_types
    WHERE property_types.name = 'string';";
pub const ADD_PATH_PROPERTY: &'static str =
"INSERT INTO vocabulary (name, type)
    SELECT $1, id FROM property_types
    WHERE property_types.name = 'path';";
pub const ADD_BOOLEAN_PROPERTY: &'static str =
"INSERT INTO vocabulary (name, type)
    SELECT $1, id FROM property_types
    WHERE property_types.name = 'bool';";
pub const ADD_DATE_PROPERTY: &'static str =
"INSERT INTO vocabulary (name, type)
    SELECT $1, id FROM property_types
    WHERE property_types.name = 'date';";
pub const ADD_TIMESTAMP_PROPERTY: &'static str =
"INSERT INTO vocabulary (name, type)
    SELECT $1, id FROM property_types
    WHERE property_types.name = 'timestamp';";
pub const ADD_INT_PROPERTY: &'static str =
"INSERT INTO vocabulary (name, type)
    SELECT $1, id FROM property_types
    WHERE property_types.name = 'int';";
pub const ADD_UINT_PROPERTY: &'static str =
"INSERT INTO vocabulary (name, type)
    SELECT $1, id FROM property_types
    WHERE property_types.name = 'uint';";
pub const ADD_FLOAT_PROPERTY: &'static str =
"INSERT INTO vocabulary (name, type)
    SELECT $1, id FROM property_types
    WHERE property_types.name = 'float';";

// TODO: FIXME!
pub const VERIFY_PROPERTY: &'static str =
"SELECT id, type FROM vocabulary
    WHERE name = $1;";

pub const SET_OBJECT_PROPERTY: &'static str =
"INSERT INTO object_properties (object_id, property, value)
    SELECT $1, id, $2 FROM vocabulary
    WHERE vocabulary.name = $3;";
