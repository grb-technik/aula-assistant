use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct FileConfig {
    security: Security,
    defaults: Defaults,
}

impl FileConfig {
    pub fn advanced_pin(&self) -> &str {
        &self.security.advanced_pin()
    }

    pub fn tablet_mode_default(&self) -> bool {
        self.defaults.tablet_mode()
    }
}

impl Default for FileConfig {
    fn default() -> Self {
        FileConfig {
            defaults: Defaults { tablet_mode: false },
            security: Security {
                advanced_pin: "1234".to_string(),
            },
        }
    }
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Defaults {
    tablet_mode: bool,
}

impl Defaults {
    pub fn tablet_mode(&self) -> bool {
        self.tablet_mode
    }
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Security {
    advanced_pin: String,
}

impl Security {
    pub fn advanced_pin(&self) -> &str {
        &self.advanced_pin
    }
}
