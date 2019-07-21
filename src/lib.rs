#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

pub mod config;
pub mod error;
pub mod loaded;
pub mod version;

use std::any::Any;

use toml::Value;

#[cfg(feature = "mammoth_module")]
pub use mammoth_macro::mammoth_module;

// TODO: complete and apply the `Log` trait to the `MammothInterface` trait.

/// Trait that contains the functions that should be implemented by a module or a handler.
pub trait MammothInterface: Any + Send + Sync {
    /// Function that is called when the library is loaded.
    fn on_load(&self, _: Option<&Value>) {}
    // FOR_LATER: load Actix crate and uncomment the following.
    // /// Function that is called during the construction of the server.
    // ///
    // /// It should output a "factory" function that can be used in `App::configure()`.
    // fn on_factory(&self, _cfg: &mut ServiceConfig) {}

    // FOR_LATER: Add Middleware support.
    // FOR_LATER: Add support for interaction between interfaces.

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
