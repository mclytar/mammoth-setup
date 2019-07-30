#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

pub mod config;
pub mod diagnostics;
pub mod error;
pub mod loaded;
pub mod version;

use std::any::Any;

use crate::diagnostics::{Log, Logger};
use crate::error::Error;

pub mod prelude {
    #[cfg(feature = "mammoth_module")]
    pub use mammoth_macro::mammoth_module;

    pub use crate::MammothInterface;
    pub use crate::error::Error;
    pub use crate::error::severity::Severity;
    pub use crate::diagnostics::{Log, Logger, AsyncLoggerReference};

    pub use toml::Value;
    pub use semver;
}

/// Trait that contains the functions that should be implemented by a module or a handler.
pub trait MammothInterface: Any + Send + Sync + Log {
    /// Function that is called when the library is loaded.
    fn on_load(&self) {}
    // FOR_LATER: load Actix crate and uncomment the following.
    // /// Function that is called during the construction of the server.
    // ///
    // /// It should output a "factory" function that can be used in `App::configure()`.
    // fn on_factory(&self, _cfg: &mut ServiceConfig) {}

    // FOR_LATER: Add Middleware support.
    // FOR_LATER: Add support for interaction between interfaces.

    /// Function that is called when the server is validating the configuration.
    fn on_validation(&self, _: &mut Logger) -> Result<(), Error>;

    /// Function that is called when the server is shut down.
    fn on_shutdown(&self) {}
}