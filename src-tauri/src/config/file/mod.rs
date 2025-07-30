// crate to parse the configuration file of the application
mod fs;
mod schema;

pub(crate) use fs::{parse_yaml, read_config_file, resolve_config_file_path};
pub(crate) use schema::FileConfig;
