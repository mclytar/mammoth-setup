//! The `ConfigurationFile` structure contains the configuration for the entire Mammoth application.

pub mod host;
pub mod mammoth;
pub mod port;
pub mod module;

use std::io::Read;
use std::fs::File;
use std::path::Path;

use toml::Value;

pub use self::host::Host;
pub use self::host::HostIdentifier;
pub use self::mammoth::Mammoth;
pub use self::module::Module;

// FOR_LATER: Add tests.
// FOR_LATER: Remove `failure` crate dependency.
// FOR_LATER: implement the `Log` trait.

/// Structure that contains all the configuration for the Mammoth application.
#[derive(Clone, Debug, Deserialize)]
pub struct ConfigurationFile {
    mammoth: Mammoth,
    #[serde(rename = "host")]
    hosts: Vec<Host>,
    #[serde(rename = "mod", default = "default_mods")]
    mods: Vec<Module>,
    environment: Option<Value>
}

#[doc(hidden)]
fn default_mods() -> Vec<Module> { Vec::new() }

impl ConfigurationFile {
    /// Creates a `ConfigurationFile` structure given a TOML file.
    pub fn from_file<P>(path: P) -> Result<ConfigurationFile, failure::Error>
        where
            P: AsRef<Path>
    {
        let mut file = File::open(path)?;
        let mut contents = String::new();

        file.read_to_string(&mut contents)?;

        Ok(toml::from_str(&contents)?)
    }
    /// Creates a `ConfigurationFile` structure given a TOML string.
    pub fn from_str(contents: &str) -> Result<ConfigurationFile, failure::Error> {
        Ok(toml::from_str(contents)?)
    }
    /// Obtains the underlying `Mammoth` structure.
    pub fn mammoth(&self) -> &Mammoth {
        &self.mammoth
    }
    pub fn mammoth_mut(&mut self) -> &mut Mammoth {
        &mut self.mammoth
    }
    /// Obtains a vector of references to the hosts.
    pub fn hosts(&self) -> Vec<&Host> {
        self.hosts.iter().collect()
    }
    /// Obtains a vector of mutable references to the hosts.
    pub fn hosts_mut(&mut self) -> Vec<&mut Host> {
        self.hosts.iter_mut().collect()
    }
    /// Adds an host.
    pub fn add_host(&mut self, host: Host) {
        self.hosts.push(host);
    }
    /// Removes an host by its id.
    pub fn remove_host(&mut self, id: HostIdentifier) {
        self.hosts.retain(|h| !h.is(&id));
    }
    /// Returns `true` if the current structure has the specified host and `false` otherwise.
    pub fn has_host(&mut self, id: HostIdentifier) -> bool {
        self.hosts.iter().position(|h| h.is(&id)).is_some()
    }

    /// Obtains a vector of references to the underlying `Module` structures defining module
    /// configuration for all hosts.
    pub fn mods(&self) -> Vec<&Module> {
        self.mods.iter().collect()
    }
    /// Obtains a vector of mutable references to the underlying `Module` structures defining module
    /// configuration for all hosts.
    pub fn mods_mut(&mut self) -> Vec<&mut Module> {
        self.mods.iter_mut().collect()
    }
    /// Adds a new module to the module list for all hosts.
    pub fn add_mod(&mut self, module: Module) {
        self.mods.push(module);
    }
    /// Removes a global module by its `name`.
    pub fn remove_mod(&mut self, name: &str) {
        self.mods.retain(|m| m.name() != name);
    }
    /// Returns `true` if the specified module is globally defined and `false` otherwise.
    pub fn has_module(&self, name: &str) -> bool {
        self.mods.iter().position(|m| m.name() == name).is_some()
    }
}