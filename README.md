# Mammoth setup

Project 'Mammoth' is an attempt to create an easy-to-use web server _executable_ in Rust.

This repository/crate is the backend library of Mammoth and contains
all the logic and server implementations.

The project is based on the [actix-web](https://github.com/actix/actix-web) framework.

## To-Do list

This is the initial To-Do list for the project.
Items beginning with [...] have more sub-tasks, possibly depending on the previous tasks.

- [ ] Add a `TOML` prototype of the possible configuration file.
- [ ] Add the basic definitions for the configuration file.
    - [x] Complete `config/mammoth/log_severity`
    - [ ] Complete `config/host`
    - [ ] Complete `config/mammoth`
    - [ ] Complete `config/module`
    - [ ] Complete `config/port`
    - [ ] Finalize
    - [ ] Version 0.0.1
- [ ] [...] Add the module handling logic.
- [ ] [...] Add the server construction logic.
- [ ] [...] Finalize the project for version 0.1.0.

## Notes

Altough I did some tests and experiments before, this project is at a very initial stage
and I am working at it in my spare time, therefore its development can be very discontinuous.

## License

[MIT](LICENSE)