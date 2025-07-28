use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Schema {}

impl Default for Schema {
    fn default() -> Self {
        Schema {}
    }
}

impl Display for Schema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Schema: {{}}")
    }
}
