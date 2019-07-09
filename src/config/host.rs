use std::path::{Path, PathBuf};

use crate::config::port::Binding;
use crate::config::module::Module;

// TODO: Add unit tests.

#[derive(Debug, Deserialize)]
pub struct Host {
    hostname: Option<String>,
    listen: Binding,
    static_dir: Option<PathBuf>,
    #[serde(default = "default_mod", rename = "mod")]
    mods: Vec<Module>
}

fn default_mod() -> Vec<Module> { Vec::new() }

impl Host {
    pub fn new(port: u16) -> Host {
        Host {
            hostname: None,
            listen: Binding::new(port),
            static_dir: None,
            mods: Vec::new()
        }
    }

    pub fn name(&self) -> Option<&str> {
        if let Some(ref name) = self.hostname { Some(name.as_str()) }
        else { None }
    }
    pub fn set_name(&mut self, name: &str) {
        self.hostname = Some(name.to_owned());
    }
    pub fn clear_name(&mut self) {
        self.hostname = None;
    }

    pub fn binding(&self) -> &Binding {
        &self.listen
    }
    pub fn binding_mut(&mut self) -> &mut Binding {
        &mut self.listen
    }
    pub fn set_binding(&mut self, port: Binding) {
        self.listen = port
    }

    pub fn serving_dir(&self) -> Option<&Path> {
        if let Some(ref path) = self.static_dir { Some(path.as_path()) }
        else { None }
    }
    pub fn set_serving_dir<P>(&mut self, path: P)
        where
            P: AsRef<Path>
    {
        self.static_dir = Some(path.as_ref().to_path_buf());
    }

    pub fn mods(&self) -> Vec<&Module> {
        self.mods.iter().collect()
    }
    pub fn mods_mut(&mut self) -> Vec<&mut Module> {
        self.mods.iter_mut().collect()
    }

    pub fn has_module(&self, name: &str) -> bool {
        for m in self.mods.iter() {
            if m.name() == name {
                return true
            }
        }

        false
    }
}