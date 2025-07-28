use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Schema {}

impl Default for Schema {
    fn default() -> Self {
        Schema {}
    }
}
