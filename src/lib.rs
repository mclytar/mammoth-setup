#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

pub mod config;
pub mod error;
pub mod loaded;

use std::any::Any;

use toml::Value;

use crate::error::validate::Validate;

/// Trait that contains the functions that should be implemented by a module or a handler.
// TODO: find the best validator type.
pub trait MammothInterface: Any + Send + Sync + Validate<Option<Value>> {
    /// Function that is called when the library is loaded.
    fn on_load(&self, _cfg: Option<&Value>) {}
    // Function that is called during the construction of the server.
    //
    // It should output a "factory" function that can be used in `App::configure()`.
    // TODO: re-doc-comment the above lines.
    // TODO: load Actix crate.
    //fn on_factory(&self, _cfg: &mut ServiceConfig) {}

    // TODO: Add Middleware support.
    // TODO: Add support for interaction between interfaces.

    /// Function that is called when the server is shut down.
    fn on_shutdown(&self) {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
