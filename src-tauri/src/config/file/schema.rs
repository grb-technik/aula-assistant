use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Schema {
    advanced_pin: String,
    tablet_mode_default: bool,
}

impl Schema {
    pub fn advanced_pin(&self) -> &str {
        &self.advanced_pin
    }

    pub fn tablet_mode_default(&self) -> bool {
        self.tablet_mode_default
    }
}

impl Default for Schema {
    fn default() -> Self {
        Schema {
            advanced_pin: "1234".to_string(),
            tablet_mode_default: false,
        }
    }
}

impl Display for Schema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Schema: {{}}")
    }
}
