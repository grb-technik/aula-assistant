use crate::config::{FileConfig, RuntimeConfig};
use std::collections::HashMap;

pub struct AppStateBuilder {
    show_appbar: Option<bool>,
    advanced_pin: Option<String>,
    artnet_bind: Option<String>,
    artnet_broadcast: Option<bool>,
    artnet_target: Option<String>,
    artnet_universe: Option<u16>,
    lighting_scenes: HashMap<String, LightingScene>,
}

impl AppStateBuilder {
    pub fn new() -> Self {
        AppStateBuilder {
            show_appbar: None,
            advanced_pin: None,
            artnet_bind: None,
            artnet_broadcast: None,
            artnet_target: None,
            artnet_universe: None,
            lighting_scenes: HashMap::new(),
        }
    }

    pub fn show_appbar(&mut self, show: bool) {
        self.show_appbar = Some(show);
    }

    pub fn advanced_pin(&mut self, pin: String) {
        self.advanced_pin = Some(pin);
    }

    pub fn build(self) -> AppState {
        AppState {
            show_appbar: self.show_appbar.expect("show_appbar must be set"),
            advanced_pin: self.advanced_pin.expect("advanced_pin must be set"),
            artnet_bind: self.artnet_bind.expect("artnet_bind must be set"),
            artnet_broadcast: self.artnet_broadcast.expect("artnet_broadcast must be set"),
            artnet_target: self.artnet_target.expect("artnet_target must be set"),
            artnet_universe: self.artnet_universe.expect("artnet_universe must be set"),
            lighting_scenes: self.lighting_scenes,
        }
    }
}

pub trait TakeFrom<T> {
    fn take_from(&mut self, config: T);
}

impl TakeFrom<&RuntimeConfig> for AppStateBuilder {
    fn take_from(&mut self, config: &RuntimeConfig) {
        if let Some(tablet_mode) = config.tablet_mode() {
            self.show_appbar(!tablet_mode);
        }
    }
}

impl TakeFrom<&FileConfig> for AppStateBuilder {
    fn take_from(&mut self, config: &FileConfig) {
        if self.show_appbar.is_none() {
            self.show_appbar(!config.tablet_mode_default());
        }

        self.advanced_pin(config.advanced_pin().to_string());

        self.artnet_bind = Some(config.lighting().artnet().bind().to_string());
        self.artnet_broadcast = Some(config.lighting().artnet().broadcast());
        self.artnet_target = Some(config.lighting().artnet().target().to_string());
        self.artnet_universe = Some(config.lighting().artnet().universe());

        let fixture_types = config.lighting().patch().types();
        let patch = config.lighting().patch().patched();

        for scene in config.lighting().scenes() {
            let scene_name = scene.name().to_string();
            let scene_type = match scene.scene_type() {
                "on" => SceneType::On,
                "off" => SceneType::Off,
                _ => SceneType::Default,
            };
            let mut channels: [Option<u8>; 512] = [None; 512];
            for set in scene.sets() {
                let fixture = patch
                    .iter()
                    .find(|f| f.name() == set.fixture())
                    .expect("Fixture not found in patch");
                let fixture_type = fixture_types
                    .iter()
                    .find(|ft| ft.name() == fixture.fixture_type())
                    .expect("Fixture type not found in types");
                let channel_index = fixture_type
                    .channels()
                    .iter()
                    .position(|c| c == set.channel())
                    .expect("Channel not found in fixture type")
                    + fixture.channel() as usize
                    - 1; // adjust for 0-based index
                let value = set.value();
                channels[channel_index] = Some(value);
            }
            let lighting_scene = LightingScene::new(scene_type, channels.to_vec());
            self.lighting_scenes.insert(scene_name, lighting_scene);
        }
    }
}

pub struct AppState {
    // defaults
    show_appbar: bool,
    // security
    advanced_pin: String,
    // lighting
    artnet_bind: String,
    artnet_broadcast: bool,
    artnet_target: String,
    artnet_universe: u16,
    lighting_scenes: HashMap<String, LightingScene>,
}

impl AppState {
    pub fn show_appbar(&self) -> bool {
        self.show_appbar
    }

    pub fn advanced_pin(&self) -> &str {
        &self.advanced_pin
    }

    pub fn artnet_bind(&self) -> &str {
        &self.artnet_bind
    }

    pub fn artnet_broadcast(&self) -> bool {
        self.artnet_broadcast
    }

    pub fn artnet_target(&self) -> &str {
        &self.artnet_target
    }

    pub fn artnet_universe(&self) -> u16 {
        self.artnet_universe
    }

    pub fn lighting_scenes(&self) -> &HashMap<String, LightingScene> {
        &self.lighting_scenes
    }
}

pub enum SceneType {
    On,
    Off,
    Default,
}

pub struct LightingScene {
    scene_type: SceneType,
    channels: Vec<Option<u8>>,
}

impl LightingScene {
    pub fn new(scene_type: SceneType, channels: Vec<Option<u8>>) -> Self {
        LightingScene {
            scene_type,
            channels,
        }
    }

    pub fn scene_type(&self) -> &SceneType {
        &self.scene_type
    }

    pub fn channels(&self) -> &Vec<Option<u8>> {
        &self.channels
    }
}
