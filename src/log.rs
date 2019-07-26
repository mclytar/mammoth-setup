use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use crate::error::Error;
use crate::error::event::Event;
use crate::error::severity::Severity;

pub type AsyncLoggerReference = Arc<RwLock<Logger>>;
pub type NoAux = ();
pub const NO_AUX: &NoAux = &();

pub trait Logger {
    fn log(&mut self, _: Severity, _: &str);
}

pub trait Validate {
    type Aux;

    fn validate(&self, _: &mut Logger, _: &Self::Aux) -> Result<(), Error>;
}

pub trait Log: Validate
{
    fn register_logger(&mut self, logger: AsyncLoggerReference);
    fn retrieve_logger(&self) -> Option<AsyncLoggerReference>;
    fn log(&self, sev: Severity, desc: &str) {
        if let Some(logger) = self.retrieve_logger() {
            let mut alr = logger.write().unwrap();

            alr.log(sev, desc);
        }
    }
}

impl Logger for Vec<Event> {
    fn log(&mut self, sev: Severity, desc: &str) {
        self.push(Event::new(sev, desc));
    }
}

#[derive(Copy, Clone)]
pub enum PathErrorKind {
    Directory,
    FileExists,
    FilePath,
}

#[derive(Copy, Clone)]
pub struct PathValidator(pub PathErrorKind, pub Severity);

impl Validate for PathBuf {
    type Aux = PathValidator;

    fn validate(&self, logger: &mut Logger, aux: &Self::Aux) -> Result<(), Error> {
        let &PathValidator(ref v, s) = aux;

        match v {
            PathErrorKind::Directory => if !self.is_dir() || !self.exists() {
                let desc = format!("Not a valid directory: '{}'.", self.display());
                logger.log(s, &desc);
                if s >= Severity::Error { Err(Error::InvalidDirectory(self.clone()))?; }
            },
            PathErrorKind::FileExists => if !self.is_file() || !self.exists() {
                let desc = format!("File does not exists: '{}'.", self.display());
                logger.log(s, &desc);
                if s >= Severity::Error { Err(Error::FileNotFound(self.clone()))?; }
            }
            PathErrorKind::FilePath => if !self.is_file() {
                let desc = format!("Not a valid file path: '{}'.", self.display());
                logger.log(s, &desc);
                if s >= Severity::Error { Err(Error::InvalidFilePath(self.clone()))?; }
            }
        }

        Ok(())
    }
}

impl<T, A> Validate for Option<T>
    where
        T: Validate<Aux=A>
{
    type Aux = A;

    fn validate(&self, logger: &mut Logger, aux: &Self::Aux) -> Result<(), Error> {
        if let Some(ref some) = self {
            some.validate(logger, aux)
        } else {
            logger.log(Severity::Information, "Nothing to validate.");
            Ok(())
        }
    }
}

impl<T> Validate for Vec<T>
    where
        T: Validate
{
    type Aux = T::Aux;

    fn validate(&self, logger: &mut Logger, aux: &Self::Aux) -> Result<(), Error> {
        self.iter().map(|e| e.validate(logger, aux)).collect()
    }
}