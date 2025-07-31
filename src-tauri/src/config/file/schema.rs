use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum FileConfigValidationError {}

impl std::fmt::Display for FileConfigValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                _ => format!("error validating file config"),
            }
        )
    }
}

impl std::error::Error for FileConfigValidationError {}

#[derive(Deserialize, Debug, Clone, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FileConfig {
    security: Security,
    defaults: Defaults,
    lighting: Lighting,
}

impl FileConfig {
    pub fn advanced_pin(&self) -> &str {
        &self.security.advanced_pin()
    }

    pub fn tablet_mode_default(&self) -> bool {
        self.defaults.tablet_mode()
    }

    pub fn validate(&self) -> Result<(), FileConfigValidationError> {
        // TODO: impl
        // - advanced_pin may only contain digits
        // - artnet bind must be a valid IP address
        // - artnet target must be a valid IP address
        // - artnet universe must be < 2^15 (even if stored as u16)
        // - patched fixtures must reference valid fixture types
        // - scenes must have valid sets
        // - scenes must have a valid type ("on", "off", "default")
        Ok(())
    }
}

impl Default for FileConfig {
    fn default() -> Self {
        FileConfig {
            defaults: Defaults { tablet_mode: false },
            security: Security {
                advanced_pin: "1234".to_string(),
            },
            lighting: Lighting {
                artnet: Artnet {
                    bind: "0.0.0.0:6454".to_string(),
                    broadcast: true,
                    target: "255.255.255.255:6454".to_string(),
                    universe: 0,
                },
                patch: Patch {
                    types: vec![],
                    patched: vec![],
                },
                scenes: vec![],
            },
        }
    }
}

#[derive(Deserialize, Debug, Clone, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Defaults {
    tablet_mode: bool,
}

impl Defaults {
    pub fn tablet_mode(&self) -> bool {
        self.tablet_mode
    }
}

#[derive(Deserialize, Debug, Clone, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Security {
    advanced_pin: String,
}

impl Security {
    pub fn advanced_pin(&self) -> &str {
        &self.advanced_pin
    }
}

#[derive(Deserialize, Debug, Clone, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Lighting {
    artnet: Artnet,
    patch: Patch,
    scenes: Vec<Scene>,
}

impl Lighting {
    pub fn artnet(&self) -> &Artnet {
        &self.artnet
    }

    pub fn patch(&self) -> &Patch {
        &self.patch
    }

    pub fn scenes(&self) -> &[Scene] {
        &self.scenes
    }
}

#[derive(Deserialize, Debug, Clone, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Artnet {
    bind: String,
    broadcast: bool,
    target: String,
    universe: u16,
}

impl Artnet {
    pub fn bind(&self) -> &str {
        &self.bind
    }

    pub fn broadcast(&self) -> bool {
        self.broadcast
    }

    pub fn target(&self) -> &str {
        &self.target
    }

    pub fn universe(&self) -> u16 {
        self.universe
    }
}

#[derive(Deserialize, Debug, Clone, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Patch {
    types: Vec<FixtureType>,
    patched: Vec<PatchedFixture>,
}

impl Patch {
    pub fn types(&self) -> &[FixtureType] {
        &self.types
    }

    pub fn patched(&self) -> &[PatchedFixture] {
        &self.patched
    }
}

#[derive(Deserialize, Debug, Clone, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FixtureType {
    name: String,
    channels: Vec<String>,
}

impl FixtureType {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn channels(&self) -> &[String] {
        &self.channels
    }
}

#[derive(Deserialize, Debug, Clone, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PatchedFixture {
    name: String,
    r#type: String,
    channel: u8,
}

impl PatchedFixture {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn r#type(&self) -> &str {
        &self.r#type
    }

    pub fn channel(&self) -> u8 {
        self.channel
    }
}

#[derive(Deserialize, Debug, Clone, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Scene {
    name: String,
    r#type: String, // "on", "off", "default"
    sets: Vec<Set>,
}

impl Scene {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn r#type(&self) -> &str {
        &self.r#type
    }

    pub fn sets(&self) -> &[Set] {
        &self.sets
    }
}

#[derive(Deserialize, Debug, Clone, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Set {
    fixture: String,
    channel: String,
    value: u8,
}

impl Set {
    pub fn fixture(&self) -> &str {
        &self.fixture
    }

    pub fn channel(&self) -> &str {
        &self.channel
    }

    pub fn value(&self) -> u8 {
        self.value
    }
}
