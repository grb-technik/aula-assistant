use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum FileConfigValidationError {
    SecurityAdvancedPinInvalid(String),
    ArtnetBindInvalid(String),
    ArtnetTargetInvalid(String),
    ArtnetUniverseInvalid(String),
    PatchedFixtureTypeInvalid(String),
    SceneTypeInvalid(String),
    SceneSetInvalid(String)
}

impl std::fmt::Display for FileConfigValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FileConfigValidationError::SecurityAdvancedPinInvalid(str) => {
                    format!("security.advanced_pin is invalid: {}", str)
                }
                FileConfigValidationError::ArtnetBindInvalid(str) => {
                    format!("lighting.artnet.bind is invalid: {}", str)
                }
                FileConfigValidationError::ArtnetTargetInvalid(str) => {
                    format!("lighting.artnet.target is invalid: {}", str)
                }
                FileConfigValidationError::ArtnetUniverseInvalid(str) => {
                    format!("lighting.artnet.universe is invalid: {}", str)
                }
                FileConfigValidationError::PatchedFixtureTypeInvalid(str) => {
                    format!("lighting.patch.patched fixture type is invalid: {}", str)
                }
                FileConfigValidationError::SceneTypeInvalid(str) => {
                    format!("lighting.scenes scene type is invalid: {}", str)
                }
                FileConfigValidationError::SceneSetInvalid(str) => {
                    format!("lighting.scenes scene set is invalid: {}", str)
                }
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
        if !self.security.advanced_pin().chars().all(char::is_numeric) {
            return Err(FileConfigValidationError::SecurityAdvancedPinInvalid(
                "advanced_pin must only contain digits".to_string(),
            ));
        } else if self.security.advanced_pin().is_empty() {
            return Err(FileConfigValidationError::SecurityAdvancedPinInvalid(
                "advanced_pin needs to be defined".to_string(),
            ));
        }

        if !self
            .lighting
            .artnet()
            .bind()
            .parse::<std::net::SocketAddr>()
            .is_ok()
        {
            return Err(FileConfigValidationError::ArtnetBindInvalid(
                "artnet bind must be a valid IP:PORT socket address".to_string(),
            ));
        }

        if !self
            .lighting
            .artnet()
            .target()
            .parse::<std::net::SocketAddr>()
            .is_ok()
        {
            return Err(FileConfigValidationError::ArtnetTargetInvalid(
                "artnet target must be a valid IP:PORT socket address".to_string(),
            ));
        }

        if self.lighting.artnet().universe > 32_768 {
            return Err(FileConfigValidationError::ArtnetUniverseInvalid(
                "artnet universe must be less than 32768".to_string(),
            ));
        }

        for fixture in self.lighting.patch().patched() {
            if !self
                .lighting
                .patch()
                .types()
                .iter()
                .any(|t| t.name() == fixture.ftype())
            {
                return Err(FileConfigValidationError::PatchedFixtureTypeInvalid(
                    format!(
                        "Patched fixture '{}' references an unknown fixture type '{}'",
                        fixture.name(),
                        fixture.ftype()
                    ),
                ));
            }
        }

        for scene in self.lighting.scenes() {
            if !["on", "off", "default"].contains(&scene.ftype()) {
                return Err(FileConfigValidationError::SceneTypeInvalid(
                    format!(
                        "Scene '{}' has an invalid type '{}', must be 'on', 'off', or 'default'",
                        scene.name(),
                        scene.ftype()
                    ),
                ));
            }

            for set in scene.sets() {
                if !self
                    .lighting
                    .patch()
                    .patched()
                    .iter()
                    .any(|f| f.name() == set.fixture())
                {
                    return Err(FileConfigValidationError::SceneSetInvalid(
                        format!(
                            "Scene '{}' references an unknown fixture '{}'",
                            scene.name(),
                            set.fixture()
                        ),
                    ));
                }

                if !self
                    .lighting
                    .patch()
                    .types()
                    .iter()
                    .any(|t| t.name() == set.channel())
                {
                    return Err(FileConfigValidationError::SceneSetInvalid(
                        format!(
                            "Scene '{}' references an unknown channel '{}'",
                            scene.name(),
                            set.channel()
                        ),
                    ));
                }
            }
        }

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
        &self.advanced_pin.trim()
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
    #[serde(rename = "type")]
    ftype: String,
    channel: u8,
}

impl PatchedFixture {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn ftype(&self) -> &str {
        &self.ftype
    }

    pub fn channel(&self) -> u8 {
        self.channel
    }
}

#[derive(Deserialize, Debug, Clone, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Scene {
    name: String,
    #[serde(rename = "type")]
    ftype: String, // "on", "off", "default"
    sets: Vec<Set>,
}

impl Scene {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn ftype(&self) -> &str {
        &self.ftype
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
