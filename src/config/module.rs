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
//!     fn validate(&self,_: &mut Logger,_: Self::Aux) -> Result<(), Error> {
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
use std::sync::Arc;

use libloading::Symbol;
use semver::Version;
use toml::Value;

use crate::MammothInterface;
use crate::error::Error;
use crate::loaded::library::LoadedModuleSet;
use crate::log::{Logger, Validate};
use crate::version;

// WARNING: untested functions.
// WARNING: `load_into` function is not tested for now (needs a library).
// FOR_LATER: Remove `failure` crate dependency.
// FOR_LATER: implement the `Validate` trait.

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
    pub fn load_into(&self, mod_set: &mut LoadedModuleSet) -> Result<(), failure::Error>
    {
        let lib_path = if let Some(ref path) = self.location {
            path.clone()
        } else {
            mod_set.lib_path(self.name())
        };

        let library = &mod_set.load(lib_path)?.library;

        let configuration = self.config.as_ref();

        let interface = unsafe {
            let constructor: Symbol<fn(Option<&Value>) -> *mut MammothInterface> = library.get(b"__construct")?;
            Arc::new(Box::from_raw(constructor(configuration)))
        };

        let version = unsafe {
            let controller: Symbol<fn() -> Version> = library.get(b"__version")?;
            controller()
        };

        if !version::compatible(&version) {
            Err(failure::err_msg("Incompatible module version."))?;
        }

        interface.on_load(self.config());

        mod_set.insert(self.name(), interface);

        Ok(())
    }
}

impl Validate for Module {
    type Aux = PathBuf;

    fn validate(&self, _logger: &mut Logger, _aux: Self::Aux) -> Result<(), Error> {
        unimplemented!()
    }
}