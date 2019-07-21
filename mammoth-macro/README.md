# Mammoth setup

Project 'Mammoth' is an attempt to create an easy-to-use web server _executable_ in Rust.

This crate contains all the useful macros for Mammoth development.

The project is based on the [actix-web](https://github.com/actix/actix-web) framework.

## How to use

Currently, attribute macros are only supported in nightly rust.
To enable the use of attribute macros, write the following at the beginning of the file.
```rust
#![feature(custom_attribute)]
```

Then, create the struct for which you want to implement the `MammothInterface` trait and apply the attribute `mammoth_module`:
```rust
// The main structure defining the module interface.
#[mammoth_module(constructor_fn)]
struct MyModule {
    /* some fields */
}

// The `MammothInterface` trait implementation.
impl MammothInterface for MyModule {
    /* implementation */
}

// The external `MyModule` constructor used to instantiate the module interface.
pub fn constructor_fn(cfg: Option<toml::Value>) -> MyModule {
    /* constructor */
}
```
Note that the `constructor_fn` function is mandatory as it is used in the `mammoth_module` macro to construct the desired structure.

## Additional notes

This is an early stage of the crate and may vary a lot.

Currently, the macro `mammoth_module` creates two entry points for the dynamic library, namely the `__construct` function to construct the internal module and the `__version` function to obtain the version of the underlying `mammoth-setup` crate.
The last one is needed in order to achieve some sort of consistency between the Mammoth application and its modules/plugins.  