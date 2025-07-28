// crate to parse the configuration file of the application
mod fs;
mod schema;
pub mod yaml;

pub use fs::read_config_file;
