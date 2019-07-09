use std::path::{PathBuf, Path};

use toml::Value;

// TODO: Add documentation.
// TODO: Add `load` function.
// TODO: Are unit tests needed here?
// TODO: Remove `failure` crate dependency.
// TODO: Perhaps add a `validate` function to validate information?

#[derive(Clone, Debug, Deserialize)]
pub struct Module {
    name: String,
    location: Option<PathBuf>,
    #[serde(default = "default_enabled")]
    enabled: bool,
    config: Option<Value>
}

fn default_enabled() -> bool { true }

impl Module {
    pub fn new(name: &str) -> Module {
        Module {
            name: name.to_owned(),
            location: None,
            enabled: true,
            config: None
        }
    }

    pub fn new_disabled(name: &str) -> Module {
        Module {
            name: name.to_owned(),
            location: None,
            enabled: false,
            config: None
        }
    }

    pub fn with_config(name: &str, enabled: bool, config: Value) -> Module
    {
        Module {
            name: name.to_owned(),
            location: None,
            enabled,
            config: Some(config)
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn config(&self) -> Option<&Value> {
        self.config.as_ref()
    }
    pub fn config_mut(&mut self) -> Option<&mut Value> {
        self.config.as_mut()
    }
    pub fn into_config(self) -> Option<Value> {
        self.config
    }

    pub fn location(&self) -> Option<&Path> {
        self.location.as_ref().and_then(|p| Some(p.as_path()))
    }
    pub fn set_location<P>(&mut self, path: P)
        where
            P: AsRef<Path>
    {
        self.location = Some(path.as_ref().to_path_buf());
    }
    pub fn clear_location(&mut self) {
        self.location = None;
    }
}