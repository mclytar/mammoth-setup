//! The `Module` structure contains the configuration for a Mammoth module.
//!
//! A 'module' is a dynamic library (`.dll` in Windows and `.so` in Linux) containing additional
//! functionality to the server.
//! The main entry point is a `__construct` function that loads all the needed configuration.
//! The simplest module is as follows.
//! ```rust
//! use mammoth_setup::MammothInterface;
//! use mammoth_setup::error::event::Event;
//! use mammoth_setup::error::validate::Validate;
//! use toml::Value;
//!
//! struct LibraryModule {
//!     /* fields omitted */
//! }
//!
//! impl Validate<Option<Value>> for LibraryModule {
//!     fn validate(&self, _: Option<Value>) -> Vec<Event> {
//!         // Validate here the configuration.
//!         unimplemented!()
//!     }
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

use libloading::{Library, Symbol};
use toml::Value;

use crate::MammothInterface;
use crate::error::event::Event;
use crate::error::validate::Validate;

// TODO: Add `load` function.
// TODO: Are unit tests needed here?
// TODO: Remove `failure` crate dependency.
// TODO: Complete `validate` function.

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
    pub fn load<P>(&self, path: P) -> Result<(), failure::Error>
        where
            P: AsRef<Path>
    {
        let lib_path = if let Some(ref path) = self.location {
            path.clone()
        } else {
            path.as_ref().join(self.name().to_owned() + ".dll")
        };

        let library = Library::new(&lib_path)?;

        let interface = unsafe {
            let constructor: Symbol<fn() -> *mut MammothInterface> = library.get(b"__construct")?;
            Arc::new(Box::from_raw(constructor()))
        };

        interface.on_load(self.config());

        unimplemented!()
    }
}

impl<V> Validate<V> for Module
    where
        V: AsRef<Path>
{
    fn validate(&self, path: V) -> Vec<Event> {
        let events = Vec::new();

        if self.enabled {
            let _lib_path = if let Some(ref path) = self.location {
                path.clone()
            } else {
                path.as_ref().join(self.name().to_owned() + ".dll")
            };

            // TODO: handle library validation.
            //let _lib = Library::new(&lib_path)?;

            // TODO: try to load the important library functions.
        }

        events
    }
}