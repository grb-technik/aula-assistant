use crate::config::error::FileValidationError;
use serde::{Deserialize, Serialize};

pub trait Validate {
    fn validate(&self) -> Result<(), FileValidationError>;
}

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

    pub fn lighting(&self) -> &Lighting {
        &self.lighting
    }
}

impl Validate for FileConfig {
    fn validate(&self) -> Result<(), FileValidationError> {
        self.defaults.validate()?;
        self.security.validate()?;
        self.lighting.validate()?;
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

impl Validate for Defaults {
    fn validate(&self) -> Result<(), FileValidationError> {
        Ok(())
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

impl Validate for Security {
    fn validate(&self) -> Result<(), FileValidationError> {
        if !self.advanced_pin().chars().all(char::is_numeric) {
            return Err(FileValidationError::SecurityAdvancedPinInvalid(
                "advanced_pin must only contain digits".to_string(),
            ));
        } else if self.advanced_pin().is_empty() {
            return Err(FileValidationError::SecurityAdvancedPinInvalid(
                "advanced_pin needs to be defined".to_string(),
            ));
        }
        Ok(())
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

impl Validate for Lighting {
    fn validate(&self) -> Result<(), FileValidationError> {
        if !self.artnet().bind().parse::<std::net::SocketAddr>().is_ok() {
            return Err(FileValidationError::ArtnetBindInvalid(
                "artnet bind must be a valid IP:PORT socket address".to_string(),
            ));
        }

        if !self
            .artnet()
            .target()
            .parse::<std::net::SocketAddr>()
            .is_ok()
        {
            return Err(FileValidationError::ArtnetTargetInvalid(
                "artnet target must be a valid IP:PORT socket address".to_string(),
            ));
        }

        if self.artnet().universe > 32_768 {
            return Err(FileValidationError::ArtnetUniverseInvalid(
                "artnet universe must be less than 32768".to_string(),
            ));
        }

        for fixture in self.patch().patched() {
            if !self
                .patch()
                .types()
                .iter()
                .any(|t| t.name() == fixture.fixture_type())
            {
                return Err(FileValidationError::PatchedFixtureTypeInvalid(format!(
                    "patched fixture '{}' references an unknown fixture type '{}'",
                    fixture.name(),
                    fixture.fixture_type()
                )));
            }
        }

        for scene in self.scenes() {
            for set in scene.sets() {
                let fixture_types = self.patch().types();
                let patch = self.patch().patched();

                let fixture = patch.iter().find(|f| f.name() == set.fixture());
                if let Some(fixture) = fixture {
                    let fixture_type = fixture.fixture_type();

                    let fixture_type_def = fixture_types.iter().find(|t| t.name() == fixture_type);
                    if let Some(fixture_type_def) = fixture_type_def {
                        if !fixture_type_def
                            .channels()
                            .contains(&set.channel().to_string())
                        {
                            return Err(FileValidationError::SceneSetInvalid(format!(
                                "scene '{}' set for fixture '{}' has an invalid channel '{}', valid channels are: {:?}",
                                scene.name(),
                                set.fixture(),
                                set.channel(),
                                fixture_type_def.channels()
                            )));
                        }
                    } else {
                        return Err(FileValidationError::SceneSetInvalid(format!(
                            "scene '{}' set for fixture '{}' references an unknown fixture type '{}'",
                            scene.name(),
                            set.fixture(),
                            fixture_type
                        )));
                    }
                } else {
                    return Err(FileValidationError::SceneSetInvalid(format!(
                        "scene '{}' references an unknown fixture '{}'",
                        scene.name(),
                        set.fixture()
                    )));
                }
            }
        }

        Ok(())
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
    fixture_type: String,
    channel: u8,
}

impl PatchedFixture {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn fixture_type(&self) -> &str {
        &self.fixture_type
    }

    pub fn channel(&self) -> u8 {
        self.channel
    }
}

#[derive(Deserialize, Debug, Clone, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Scene {
    name: String,
    sets: Vec<Set>,
}

impl Scene {
    pub fn name(&self) -> &str {
        &self.name
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
