use crate::config::{ConfigError, file::schema::Schema};

pub fn parse_yaml(yaml: &str) -> Result<Schema, ConfigError> {
    match serde_yaml::from_str(yaml) {
        Ok(parsed) => return Ok(parsed),
        Err(e) => return Err(ConfigError::YamlError(e)),
    }
}
