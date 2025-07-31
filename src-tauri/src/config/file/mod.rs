// crate to parse the configuration file of the application
mod lib;
mod schema;

pub(crate) use lib::{parse_yaml, open_config_file_as_reader, resolve_config_file_path};
pub(crate) use schema::FileConfig;
