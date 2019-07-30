//! The `Module` structure contains the configuration for a Mammoth module.
//!
//! A 'module' is a dynamic library (`.dll` in Windows and `.so` in Linux) containing additional
//! functionality to the server.
//! The main entry point is a `__construct` function that loads all the needed configuration.
//! The simplest module is as follows.
//! ```rust
//! use mammoth_setup::MammothInterface;
//! use mammoth_setup::diagnostics::{Log, Logger};
//! use mammoth_setup::error::Error;
//! use toml::Value;
//!
//! struct LibraryModule {
//!     /* fields omitted */
//! }
//!
//! impl Log for LibraryModule {
//!     /* implementation omitted */
//! #    fn register_logger(&mut self,logger: std::sync::Arc<std::sync::RwLock<Logger>>) {
//! #        unimplemented!()
//! #    }
//! #    fn retrieve_logger(&self) -> Option<std::sync::Arc<std::sync::RwLock<Logger>>> {
//! #        unimplemented!()
//! #    }
//! }
//!
//! impl MammothInterface for LibraryModule {
//! #    fn on_validation(&self,_: &mut Logger) -> Result<(), Error> {
//! #        unimplemented!()
//! #    }
//!     /* implementation omitted */
//! }
//!
//! #[no_mangle]
//! fn __construct() -> *mut MammothInterface {
//!     let interface = LibraryModule { /* ... */ };
//!     /* initialization omitted */
//!     let interface = Box::new(interface);
//!     let interface = Box::into_raw(interface);
//!     interface
//! }
//! ```
//!
//! There may be other available entry points in the future (probably, at least a `__version`
//! function and a `__validate` function).

use std::path::{PathBuf, Path};
use std::str::FromStr;
use std::sync::Arc;

use libloading::{Library, Symbol};
use semver::{Version, VersionReq};
use toml::Value;

use crate::MammothInterface;
use crate::loaded::library::LoadedModuleSet;
use crate::diagnostics::{Id, Logger, Validator};
use crate::error::Error;
use crate::error::severity::Severity;
use crate::version;

#[cfg(target_os="windows")]
pub(crate) const DYLIB_EXT: &str = ".dll";
#[cfg(target_os="linux")]
pub(crate) const DYLIB_EXT: &str = ".so";

/// Structure that defines configuration for a module library.
#[derive(Clone, Debug, Deserialize)]
pub struct Module {
    name: String,
    location: Option<PathBuf>,
    #[serde(default = "default_enabled")]
    enabled: bool,
    config: Option<Value>
}

#[doc(hidden)]
fn default_enabled() -> bool { true }

impl Module {
    /// Creates a new `Module` structure given its name.
    pub fn new(name: &str) -> Module {
        Module {
            name: name.to_owned(),
            location: None,
            enabled: true,
            config: None
        }
    }
    /// Creates a new, disabled `Module` structure given its name.
    pub fn new_disabled(name: &str) -> Module {
        Module {
            name: name.to_owned(),
            location: None,
            enabled: false,
            config: None
        }
    }
    /// Creates a new `Module` structure given its name and configuration.
    pub fn with_config(name: &str, enabled: bool, config: Value) -> Module
    {
        Module {
            name: name.to_owned(),
            location: None,
            enabled,
            config: Some(config)
        }
    }
    /// Obtains the name of the module.
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Enables the module.
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    /// Disables the module.
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    /// Returns `true` if the module is enabled and `false` otherwise.
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    /// Returns a reference to the `TOML` module configuration, if any.
    pub fn config(&self) -> Option<&Value> {
        self.config.as_ref()
    }
    /// Returns a mutable reference to the `TOML` module configuration, if any.
    pub fn config_mut(&mut self) -> Option<&mut Value> {
        self.config.as_mut()
    }
    /// Transforms the current `Module` structure into its `TOML` configuration, if any.
    pub fn into_config(self) -> Option<Value> {
        self.config
    }

    /// Returns the path of the library containing this module, if any.
    ///
    /// If no location is given, this function returns `None` and Mammoth uses the default module
    /// directory.
    pub fn location(&self) -> Option<&Path> {
        self.location.as_ref().and_then(|p| Some(p.as_path()))
    }
    /// Sets the path of the library containing this module.
    pub fn set_location<P>(&mut self, path: P)
        where
            P: AsRef<Path>
    {
        self.location = Some(path.as_ref().to_path_buf());
    }
    /// Removes the given path of the library containing this module.
    pub fn clear_location(&mut self) {
        self.location = None;
    }
    /// Tries to load the library.
    pub fn load_into(&self, mod_set: &mut LoadedModuleSet) -> Result<(), Error>
    {
        let lib_path = if let Some(ref path) = self.location {
            path.clone()
        } else {
            mod_set.lib_path(self.name())
        };

        let library = &mod_set.load(lib_path)?.library;

        let version = unsafe {
            let controller: Symbol<extern fn() -> Version> = library.get(b"__version")?;
            controller()
        };

        if !version::compatible(&version) {
            Err(Error::InvalidModuleVersion(version.clone(), VersionReq::from_str(version::COMPATIBILITY_STRING).unwrap()))?;
        }

        let configuration = self.config.clone();

        let interface = unsafe {
            let constructor: Symbol<extern fn(Option<Value>) -> *mut MammothInterface> = library.get(b"__construct")?;
            Arc::new(Box::from_raw(constructor(configuration)))
        };

        interface.on_load();

        mod_set.insert(self.name(), interface);

        Ok(())
    }
}

impl Id for Module {
    type Identifier = String;

    fn id(&self) -> Self::Identifier {
        self.name.to_owned()
    }
}

impl Validator<Module> for PathBuf {
    fn validate(&self, logger: &mut Logger, item: &Module) -> Result<(), Error> {
        let filename = if let Some(filename) = item.location() {
            filename.to_path_buf()
        } else {
            self.join(item.name().to_owned() + DYLIB_EXT)
        };
        let lib = Library::new(&filename)?;
        let ver: Version = unsafe {
            let ver_fn: Symbol<extern fn() -> Version> = lib.get(b"__version")?;
            ver_fn()
        };

        if !version::compatible(&ver) {
            let desc = format!("Incompatible module version for '{}': {}. Must respect requisite {}.", item.name(), &ver, version::COMPATIBILITY_STRING);
            logger.log(Severity::Critical, &desc);
            Err(Error::InvalidModuleVersion(ver.clone(), VersionReq::from_str(version::COMPATIBILITY_STRING).unwrap()))?;
        }

        let configuration = if let Some(config) = item.config() {
            Some(config.to_owned())
        } else {
            None
        };

        let interface: Box<MammothInterface> = unsafe {
            let constructor: Symbol<extern fn(Option<Value>) -> *mut MammothInterface> = lib.get(b"__construct")?;
            Box::from_raw(constructor(configuration))
        };

        interface.on_validation(logger)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;
    use std::str::FromStr;

    use toml::Value;

    use crate::config::Module;
    use crate::error::event::Event;
    use crate::loaded::library::LoadedModuleSet;
    use crate::diagnostics::Validator;

    #[test]
    /// Tests `Module` properties.
    fn test_generic_properties() {
        let mut module = Module::new("mod_test");
        let module_disabled = Module::new_disabled("mod_disabled");
        let module_with_config = Module::with_config("mod_configured", true, Value::from(42));

        assert_eq!(module.name(), "mod_test");
        assert_eq!(module.location(), None);
        assert_eq!(module.enabled(), true);
        assert_eq!(module.config(), None);

        assert_eq!(module_disabled.name(), "mod_disabled");
        assert_eq!(module_disabled.location(), None);
        assert_eq!(module_disabled.enabled(), false);
        assert_eq!(module_disabled.config(), None);

        assert_eq!(module_with_config.name(), "mod_configured");
        assert_eq!(module_with_config.location(), None);
        assert_eq!(module_with_config.enabled(), true);
        assert_eq!(module_with_config.config(), Some(&Value::from(42)));

        module.set_location("./target/debug/mod_test.dll");
        let location = module.location().unwrap().to_str().unwrap();
        assert_eq!(location, "./target/debug/mod_test.dll");
        module.clear_location();
        assert_eq!(module.location(), None);

        module.disable();
        assert_eq!(module.enabled(), false);
        module.enable();
        assert_eq!(module.enabled(), true);
    }

    #[test]
    /// Tests module loading.
    fn test_module_load_into() {
        let module = Module::new("mod_test");
        let mut lms = LoadedModuleSet::new("./target/debug/");

        module.load_into(&mut lms).unwrap();
    }

    #[test]
    /// Tests module validation.
    fn test_module_validation() {
        let validator = PathBuf::from_str("./target/debug/").unwrap();
        let module = Module::new("mod_test");
        let mut events: Vec<Event> = Vec::new();

        validator.validate(&mut events, &module).unwrap();
    }

    #[test]
    /// Tests module validation resulting in error.
    fn test_err_module_validation() {
        let validator = PathBuf::from_str("./target/debug/").unwrap();
        let configuration = Value::from("test_error");
        let module = Module::with_config("mod_test", true, configuration);
        let mut events: Vec<Event> = Vec::new();

        assert!(validator.validate(&mut events, &module).is_err());
    }
}