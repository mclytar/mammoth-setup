# Mammoth setup

Project 'Mammoth' is an attempt to create an easy-to-use web server _executable_ in Rust.

This repository/crate is the backend library of Mammoth and contains
all the logic and server implementations.

The project uses the [actix-web](https://github.com/actix/actix-web) framework.

## To-Do list

This is the initial To-Do list for the project.
Items beginning with [...] have more hidden/non-definitive sub-tasks, possibly depending on the previous tasks.

- [x] Add a `TOML` prototype of the possible configuration file.
- [x] Add the basic definitions for the configuration file.
    - [x] Complete `error/severity`.
    - [x] Complete `config`.
    - [x] Complete `config/host`.
    - [x] Complete `config/mammoth`.
    - [x] Complete `config/module`.
    - [x] Complete `config/port`.
    - [x] Finalize.
    - [x] Version 0.0.1.
- [ ] Add error management.
    - [x] Add `Log` and `Logger` traits.
    - [x] Add `Validate` trait.
    - [ ] Add `Id` trait and Id uniqueness validation.
    - [ ] Complete the `Error` enum definition.
    - [ ] Finalize.
    - [ ] Version 0.0.2.
        - [ ] (Complete _Module handling logic_).
- [ ] [...] Add the module handling logic.
    - [ ] [...] Add version control system for dynamic libraries.
    - [ ] Version 0.0.2.
        - [ ] (Complete _Error management_).
- [ ] [...] Add the server construction logic.
- [ ] [...] Finalize the project for version 0.1.0.

Additional and specific To-Do tasks can be found directly into the source code.

## Additional notes

Although I did some tests and experiments before, this project is at a very initial stage
and I am working at it in my spare time, therefore its development can be very discontinuous.

This repository starts as an "official" refactor of some previous attempts and sketches.

### Modules

Mammoth modules are "plug-in" dynamic libraries that contain specific functions.
The module question is delicate and requires some `unsafe` code and the creation of a
version control system to avoid conflicts (and possible unexpected or problematic
situations) between the same structures having different implementations.
Once this project reaches a "stable alpha" version, I will also add more checks and create
a template-module repository in order to ease module creation.

## License

[MIT](LICENSE)