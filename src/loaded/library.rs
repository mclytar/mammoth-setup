use std::path::{Path, PathBuf};
use std::sync::Arc;

use libloading::Library;

use crate::MammothInterface;
use crate::error::Error;
use crate::id::Id;

pub struct LoadedLibrary {
    pub path: PathBuf,
    pub library: Library
}

impl Id for LoadedLibrary {
    type Identifier = PathBuf;

    fn id(&self) -> Self::Identifier {
        self.path.clone()
    }
}

#[allow(dead_code)]
pub struct LoadedModule {
    pub(in self) library: Arc<String>,
    pub(in self) interface: Arc<Box<MammothInterface>>
}

pub struct LoadedModuleSet {
    default_path: PathBuf,
    libraries: Vec<Arc<LoadedLibrary>>,
    modules: Vec<Arc<LoadedModule>>
}

impl LoadedModuleSet {
    pub fn new<P>(default_path: P) -> LoadedModuleSet
        where
            P: AsRef<Path>
    {
        LoadedModuleSet {
            default_path: default_path.as_ref().to_path_buf(),
            libraries: Vec::new(),
            modules: Vec::new()
        }
    }

    pub fn load<P>(&mut self, path: P) -> Result<Arc<LoadedLibrary>, Error>
        where
            P: AsRef<Path>
    {
        let path = path.as_ref();
        let lib = self.libraries.iter().find(|e| e.path == path);

        if let Some(lib) = lib {
            Ok(lib.clone())
        } else {
            let library = Library::new(path)?;
            let path = path.to_path_buf();
            let loaded = Arc::new(LoadedLibrary { path, library });
            self.libraries.push(loaded.clone());
            Ok(loaded)
        }
    }

    pub fn lib_path(&self, name: &str) -> PathBuf
    {
        self.default_path.join(name.to_owned() + ".dll")
    }

    pub fn insert(&mut self, name: &str, interface: Arc<Box<MammothInterface>>) {
        self.modules.push(Arc::new(LoadedModule{
            library: Arc::new(name.to_owned()),
            interface
        }));
    }
}