//! The `ConfigurationFile` structure contains the configuration for the entire Mammoth application.

pub mod host;
pub mod mammoth;
pub mod port;
pub mod module;

use std::io::Read;
use std::fs::File;
use std::marker::PhantomData;
use std::path::Path;

use toml::Value;

pub use self::host::Host;
pub use self::host::HostIdentifier;
pub use self::mammoth::Mammoth;
pub use self::module::Module;
use crate::diagnostics::{Validator, IdValidator};
use crate::diagnostics::Logger;
use crate::error::Error;
use crate::error::severity::Severity;

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
    pub fn from_file<P>(path: P) -> Result<ConfigurationFile, Error>
        where
            P: AsRef<Path>
    {
        let mut file = File::open(path)?;
        let mut contents = String::new();

        file.read_to_string(&mut contents)?;

        Ok(toml::from_str(&contents)?)
    }
    /// Creates a `ConfigurationFile` structure given a TOML string.
    pub fn from_str(contents: &str) -> Result<ConfigurationFile, Error> {
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
    pub fn has_host(&self, id: HostIdentifier) -> bool {
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

impl Validator<ConfigurationFile> for () {
    fn validate(&self, logger: &mut Logger, item: &ConfigurationFile) -> Result<(), Error> {
        ().validate(logger, item.mammoth())?;

        if item.hosts().is_empty() {
            logger.log(Severity::Critical, "No host specified.");
            Err(Error::NoHost)?;
        }

        let mods_dir = item.mammoth().mods_dir();
        if let Some(mods_dir) = mods_dir {
            IdValidator(Severity::Critical, mods_dir.to_path_buf(), PhantomData)
                .validate(logger, &item.mods())?;
            IdValidator(Severity::Critical, mods_dir.to_path_buf(), PhantomData)
                .validate(logger, &item.hosts())?;
        } else {
            if !item.mods().is_empty() {
                logger.log(Severity::Critical, "Enabled modules without specifying modules directory.");
                Err(Error::NoModsDir)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::config::{ConfigurationFile, HostIdentifier};
    use crate::error::Error;
    use crate::error::event::Event;
    use crate::diagnostics::Validator;

    #[test]
    /// Tests a common configuration file.
    fn test_config() {
        let configuration = ConfigurationFile::from_file("./tests/test_config.toml").unwrap();
        let mut events: Vec<Event> = Vec::new();

        ().validate(&mut events, &configuration).unwrap();
    }

    #[test]
    /// Tests a common configuration file with an error flag set in the configuration of the `mod_test` module.
    fn test_config_bad_mod() {
        let configuration = ConfigurationFile::from_file("./tests/test_config_bad_mod.toml").unwrap();
        let mut events: Vec<Event> = Vec::new();

        let err = ().validate(&mut events, &configuration).unwrap_err();

        match err {
            Error::Unknown => {},
            _ => { panic!("Should be 'Unknown' error generated in module validation."); }
        }
    }

    #[test]
    /// Tests a minimal configuration TOML.
    fn test_config_minimal() {
        let toml = r##"
        [mammoth]

        [[host]]
        listen = 8080
        "##;
        let configuration = ConfigurationFile::from_str(toml).unwrap();
        let mut events: Vec<Event> = Vec::new();

        ().validate(&mut events, &configuration).unwrap();
    }

    #[test]
    /// Tests for the `NoModsDir` error when a module is specified without specifying the modules directory.
    fn test_config_no_mod_error() {
        let toml = r##"
        [mammoth]

        [[host]]
        listen = 8080

        [[mod]]
        name = "mod_test"
        "##;
        let configuration = ConfigurationFile::from_str(toml).unwrap();
        let mut events: Vec<Event> = Vec::new();

        let err = ().validate(&mut events, &configuration).unwrap_err();

        match err {
            Error::NoModsDir => {},
            _ => { panic!("Should be 'NoModsDir' error."); }
        }
    }

    #[test]
    /// Tests the `has_host` and `remove_host` functions.
    fn test_hosts() {
        let toml = r##"
        [mammoth]

        [[host]]
        hostname = "localhost"
        listen = 8080

        [[host]]
        hostname = "127.0.0.1"
        listen = 8080

        [[host]]
        listen = 8080

        [[host]]
        listen = 8088
        "##;
        let mut configuration = ConfigurationFile::from_str(toml).unwrap();

        assert!(configuration.has_host(HostIdentifier::new(8080, Some("localhost"))));
        assert!(configuration.has_host(HostIdentifier::new(8080, Some("127.0.0.1"))));
        assert!(configuration.has_host(HostIdentifier::new(8080, None)));

        assert!(!configuration.has_host(HostIdentifier::new(8443, Some("localhost"))));
        assert!(!configuration.has_host(HostIdentifier::new(8443, None)));
        assert!(!configuration.has_host(HostIdentifier::new(8080, Some("0.0.0.0"))));

        assert!(configuration.has_host(HostIdentifier::new(8088, None)));
        configuration.remove_host(HostIdentifier::new(8088, None));
        assert!(!configuration.has_host(HostIdentifier::new(8088, None)));
    }

    #[test]
    /// Tests the `has_module` and `remove_mod` functions.
    fn test_mods() {
        let toml = r##"
        [mammoth]
        mods_dir = "./mods/"

        [[host]]
        listen = 8080

        [[mod]]
        name = "mod_test"

        [[mod]]
        name = "mod_dummy"
        "##;
        let mut configuration = ConfigurationFile::from_str(toml).unwrap();

        assert!(configuration.has_module("mod_test"));
        assert!(!configuration.has_module("mod_nope"));

        assert!(configuration.has_module("mod_dummy"));
        configuration.remove_mod("mod_dummy");
        assert!(!configuration.has_module("mod_dummy"));
    }
}