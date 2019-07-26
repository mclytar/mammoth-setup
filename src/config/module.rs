//! The `Module` structure contains the configuration for a Mammoth module.
//!
//! A 'module' is a dynamic library (`.dll` in Windows and `.so` in Linux) containing additional
//! functionality to the server.
//! The main entry point is a `__construct` function that loads all the needed configuration.
//! The simplest module is as follows.
//! ```rust
//! use mammoth_setup::MammothInterface;
//! use mammoth_setup::error::Error;
//! use mammoth_setup::log::{Log, Logger, Validate};
//! use toml::Value;
//!
//! struct LibraryModule {
//!     /* fields omitted */
//! }
//!
//! impl Validate for LibraryModule {
//!     type Aux = Option<Value>;
//!
//!     fn validate(&self,_: &mut Logger,_: &Self::Aux) -> Result<(), Error> {
//!         // This function checks that the given configuration is correct.
//!         Ok(())
//!     }
//! }
//!
//! impl Log for LibraryModule {
//!     /* Logger registration omitted */
//! #    fn register_logger(&mut self,logger: std::sync::Arc<std::sync::RwLock<Logger>>) {
//! #        unimplemented!()
//! #    }
//! #    fn retrieve_logger(&self) -> Option<std::sync::Arc<std::sync::RwLock<Logger>>> {
//! #        unimplemented!()
//! #    }
//! }
//!
//! impl MammothInterface for LibraryModule {
//!     /* implementation omitted */
//! }
//!
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
use crate::error::Error;
use crate::loaded::library::LoadedModuleSet;
use crate::log::{Logger, Validate};
use crate::version;
use crate::error::severity::Severity;
use crate::id::Id;

// WARNING: untested functions.
// WARNING: `load_into` function is not tested for now (needs a library).

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
            let controller: Symbol<fn() -> Version> = library.get(b"__version")?;
            controller()
        };

        if !version::compatible(&version) {
            Err(Error::InvalidModuleVersion(version.clone(), VersionReq::from_str(version::COMPATIBILITY_STRING).unwrap()))?;
        }

        let configuration = self.config.as_ref();

        let interface = unsafe {
            let constructor: Symbol<fn(Option<&Value>) -> *mut MammothInterface> = library.get(b"__construct")?;
            Arc::new(Box::from_raw(constructor(configuration)))
        };

        interface.on_load(self.config());

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

impl Validate for Module {
    type Aux = PathBuf;

    fn validate(&self, logger: &mut Logger, aux: &Self::Aux) -> Result<(), Error> {
        let lib = Library::new(aux)?;
        let ver = unsafe {
            let ver_fn: Symbol<fn() -> Version> = lib.get(b"__version")?;
            ver_fn()
        };

        if !version::compatible(&ver) {
            let desc = format!("Incompatible module version for '{}': {}. Must respect requisite {}.", &self.name, &ver, version::COMPATIBILITY_STRING);
            logger.log(Severity::Critical, &desc);
            Err(Error::InvalidModuleVersion(ver.clone(), VersionReq::from_str(version::COMPATIBILITY_STRING).unwrap()))?;
        }

        let interface = unsafe {
            let constructor: Symbol<fn() -> *mut MammothInterface> = lib.get(b"__construct")?;
            Box::from_raw(constructor())
        };

        interface.validate(logger, &self.config)
    }
}