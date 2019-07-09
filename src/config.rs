pub mod host;
pub mod mammoth;
pub mod port;
pub mod module;

use std::io::Read;
use std::fs::File;
use std::path::{PathBuf, Path};

use toml::Value;

use self::mammoth::Mammoth;
use self::host::Host;
use self::module::Module;

// TODO: Implement `ConfigurationFile`
// TODO: Add tests.
// TODO: Remove `failure` crate dependency.

#[derive(Debug, Deserialize)]
pub struct ConfigurationFile {
    mammoth: Mammoth,
    #[serde(rename = "host")]
    hosts: Vec<Host>,
    #[serde(rename = "mod", default = "default_mods")]
    mods: Vec<Module>,
    environment: Option<Value>
}

fn default_mods() -> Vec<Module> { Vec::new() }

impl ConfigurationFile {
    pub fn from_file<P>(path: P) -> Result<ConfigurationFile, failure::Error>
        where
            P: AsRef<Path>
    {
        let mut file = File::open(path)?;
        let mut contents = String::new();

        file.read_to_string(&mut contents);

        Ok(toml::from_str(&contents)?)
    }

    pub fn from_str(contents: &str) -> Result<ConfigurationFile, failure::Error> {
        Ok(toml::from_str(contents)?)
    }
}