//! The `Host` structure contains the configuration for a host.
//!
//! The `HostIdentifier` structure contains information that uniquely identifies an host in the
//! configuration file.
//! Note that an `HostIdentifier` does not uniquely identify the configuration related to that host,
//! but only the port/hostname pair.
//!
//! Only one host is allowed per port/hostname pair.
use std::marker::PhantomData;
use std::path::{Path, PathBuf};

use regex::Regex;

use crate::config::module::Module;
use crate::config::port::Binding;
use crate::diagnostics::{Id, IdValidator, Logger, PathValidator, PathValidatorKind, Validator};
use crate::error::Error;
use crate::error::severity::Severity;

const REGEX_NAME_ADDRESS_STRING: &str = r#"^(([a-zA-Z0-9]|[a-zA-Z0-9][a-zA-Z0-9\-]*[a-zA-Z0-9])\.)*([A-Za-z0-9]|[A-Za-z0-9][A-Za-z0-9\-]*[A-Za-z0-9])$"#;
const REGEX_IP_ADDRESS_STRING: &str = r#"^(([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])\.){3}([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])$"#;

/// Structure that uniquely identifies an `Host` structure within a vector of hosts.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct HostIdentifier {
    hostname: Option<String>,
    port: u16
}

/// Structure that defines configuration for a host.
#[derive(Clone, Debug, Deserialize)]
pub struct Host {
    hostname: Option<String>,
    listen: Binding,
    static_dir: Option<PathBuf>,
    #[serde(default = "default_mod", rename = "mod")]
    mods: Vec<Module>
}

#[doc(hidden)]
fn default_mod() -> Vec<Module> { Vec::new() }

impl HostIdentifier {
    /// Creates a new `HostIdentifier` structure containing the port and the host name, if any.
    pub fn new(port: u16, name: Option<&str>) -> HostIdentifier {
        HostIdentifier {
            hostname: name.and_then(|s| Some(s.to_owned())),
            port
        }
    }
    /// Retrieves the port of the identified host.
    pub fn port(&self) -> u16 {
        self.port
    }
    /// Retrieves the host name of the identified host.
    pub fn name(&self) -> Option<&str> {
        if let Some(ref name) = self.hostname {
            Some(name)
        } else {
            None
        }
    }
}

impl Host {
    /// Creates a new `Host` structure with a binding on the specified `port`.
    pub fn new(port: u16) -> Host {
        Host {
            hostname: None,
            listen: Binding::new(port),
            static_dir: None,
            mods: Vec::new()
        }
    }
    /// Creates a new `Host` structure with a secure binding on the specified `port` and the
    /// specified `cert`, `key`.
    pub fn with_security<P, Q>(port: u16, cert: P, key: Q) -> Host
        where
            P: AsRef<Path>,
            Q: AsRef<Path>
    {
        Host {
            hostname: None,
            listen: Binding::with_security(port, cert, key),
            static_dir: None,
            mods: Vec::new()
        }
    }

    /// Obtains an identifier that uniquely identifies the host in the configuration file.
    pub fn identifier(&self) -> HostIdentifier {
        HostIdentifier::new(self.listen.port(), self.name())
    }
    /// Returns `true` if the current host corresponds to the given identifier `id` and `false`
    /// otherwise.
    pub fn is(&self, id: &HostIdentifier) -> bool {
        self.listen.port() == id.port() && self.name() == id.name()
    }

    /// Obtains the `hostname` of the host.
    pub fn name(&self) -> Option<&str> {
        if let Some(ref name) = self.hostname { Some(name.as_str()) }
        else { None }
    }
    /// Sets the `hostname` of the host.
    pub fn set_name(&mut self, name: &str) {
        self.hostname = Some(name.to_owned());
    }
    /// Clears the `hostname` of the host.
    pub fn clear_name(&mut self) {
        self.hostname = None;
    }

    /// Obtains a reference to the underlying `Binding` structure that defines the binding for the
    /// current host.
    pub fn binding(&self) -> &Binding {
        &self.listen
    }
    /// Obtains a mutable reference to the underlying `Binding` structure that defines the binding
    /// for the current host.
    pub fn binding_mut(&mut self) -> &mut Binding {
        &mut self.listen
    }
    /// Replaces the underlying `Binding` structure with a new one specified in `binding`
    pub fn set_binding(&mut self, binding: Binding) {
        self.listen = binding
    }

    /// Obtains the current serving directory, if any.
    pub fn serving_dir(&self) -> Option<&Path> {
        if let Some(ref path) = self.static_dir { Some(path.as_path()) }
        else { None }
    }
    /// Sets the serving directory for the host.
    pub fn set_serving_dir<P>(&mut self, path: P)
        where
            P: AsRef<Path>
    {
        self.static_dir = Some(path.as_ref().to_path_buf());
    }
    /// Removes the current serving directory from the host.
    pub fn clear_serving_dir(&mut self) {
        self.static_dir = None;
    }

    /// Obtains a vector of references to the underlying `Module` structures defining module
    /// configuration for this host.
    pub fn mods(&self) -> Vec<&Module> {
        self.mods.iter().collect()
    }
    /// Obtains a vector of mutable references to the underlying `Module` structures defining module
    /// configuration for this host.
    pub fn mods_mut(&mut self) -> Vec<&mut Module> {
        self.mods.iter_mut().collect()
    }
    /// Adds a new module to the module list for this host.
    pub fn add_mod(&mut self, module: Module) {
        self.mods.push(module);
    }
    /// Removes a module for this host by its `name`.
    pub fn remove_mod(&mut self, name: &str) {
        self.mods.retain(|m| m.name() != name);
    }
    /// Returns `true` if the host has the specified module and `false` otherwise.
    pub fn has_module(&self, name: &str) -> bool {
        for m in self.mods.iter() {
            if m.name() == name {
                return true
            }
        }

        false
    }
}

impl Id for Host {
    type Identifier = HostIdentifier;

    fn id(&self) -> Self::Identifier {
        HostIdentifier::new(self.listen.port(), self.name())
    }
}

impl Validator<Host> for PathBuf {
    fn validate(&self, logger: &mut Logger, item: &Host) -> Result<(), Error> {
        lazy_static! {
            static ref RE_IP: Regex = Regex::new(REGEX_IP_ADDRESS_STRING).unwrap();
            static ref RE_ADDR: Regex = Regex::new(REGEX_NAME_ADDRESS_STRING).unwrap();
        }

        ().validate(logger, item.binding())?;

        if let Some(name) = item.name() {
            if !RE_IP.is_match(name) && !RE_ADDR.is_match(name) {
                let desc = format!("Invalid hostname: '{}'.", name);
                logger.log(Severity::Critical, &desc);
                Err(Error::InvalidHostname(name.to_owned()))?;
            }
        }

        if let Some(serving_dir) = item.serving_dir() {
            PathValidator(Severity::Error, PathValidatorKind::ExistingDirectory)
                .validate(logger, &serving_dir)?;
        }

        let validator = IdValidator(Severity::Critical, self.clone(), PhantomData);
        validator.validate(logger, &item.mods())?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::path::{Path, PathBuf};

    use crate::config::host::Host;
    use crate::config::module::Module;
    use crate::config::port::Binding;
    use crate::error::event::Event;

    #[test]
    /// Tests binding.
    fn test_binding() {
        let mut host = Host::new(80);
        let binding = Binding::new(80);
        let binding_ssl = Binding::with_security(443, "./cert.pem", "./key.pem");
        assert_eq!(host.binding(), &binding);

        host.set_binding(binding_ssl.clone());
        assert_eq!(host.binding(), &binding_ssl);
    }

    #[test]
    /// Tests hostname.
    fn test_host_name() {
        let mut host = Host::new(80);
        assert!(host.name().is_none());

        host.set_name("localhost");
        assert_eq!(host.name().unwrap(), "localhost");

        host.clear_name();
        assert!(host.name().is_none());
    }

    #[test]
    /// Tests serving dir.
    fn test_serving_dir() {
        let mut host = Host::new(80);
        assert!(host.serving_dir().is_none());

        host.set_serving_dir("./www/");
        assert_eq!(host.serving_dir().unwrap(), Path::new("./www/"));

        host.clear_serving_dir();
        assert!(host.serving_dir().is_none());
    }

    #[test]
    /// Tests the `has_module` function.
    fn test_has_module() {
        let mut host = Host::new(80);
        let module = Module::new("mod_test");
        assert_eq!(host.has_module("mod_test"), false);

        host.add_mod(module);
        assert_eq!(host.has_module("mod_test"), true);
    }

    #[test]
    /// Tests the `remove` function for removing modules.
    fn test_remove_mod() {
        let mut host = Host::new(80);

        host.add_mod(Module::new("mod_dummy"));
        host.add_mod(Module::new("mod_test"));

        assert_eq!(host.has_module("mod_dummy"), true);
        assert_eq!(host.has_module("mod_test"), true);

        host.remove_mod("mod_dummy");

        assert_eq!(host.has_module("mod_dummy"), false);
        assert_eq!(host.has_module("mod_test"), true);
    }

    #[test]
    /// Tests the `validate` function.
    fn test_validate() {
        use crate::diagnostics::Validator;
        use std::str::FromStr;
        let host = Host::new(80);
        let host_ssl = Host::with_security(443, "./tests/test_cert.pem", "./tests/test_key.pem");
        let host_err = Host::with_security(443, "./err_cert.pem", "./err_key.pem");
        let mut host_named = Host::new(80);
        let mut host_named_err = Host::new(80);
        host_named.set_name("localhost");
        host_named_err.set_name("invalid@name");

        let mut events: Vec<Event> = Vec::new();
        let path_buf = PathBuf::from_str("./mods/").unwrap();

        assert!(path_buf.validate(&mut events, &host).is_ok());
        assert!(path_buf.validate(&mut events, &host_ssl).is_ok());
        assert!(path_buf.validate(&mut events, &host_err).is_err());
        assert!(path_buf.validate(&mut events, &host_named).is_ok());
        assert!(path_buf.validate(&mut events, &host_named_err).is_err());
    }
}