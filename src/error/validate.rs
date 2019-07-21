use std::path::PathBuf;

use super::Error;
use super::event::Event;
use super::severity::Severity;

pub trait Id {
    type Index: Eq;

    fn id(&self) -> Self::Index;
}

impl<T> Id for T
    where
        T: Eq + Copy
{
    type Index = Self;

    fn id(&self) -> Self::Index {
        *self
    }
}

pub trait Validate<V> {
    fn validate(&self, _: V) -> Vec<Event>;
}

pub enum PathErrorKind {
    Directory,
    FileExists,
    FilePath,
}

pub struct PathValidator(pub PathErrorKind, pub Severity);

impl Validate<PathValidator> for PathBuf {
    fn validate(&self, validator: PathValidator) -> Vec<Event> {
        let mut events = Vec::new();
        let PathValidator(v, s) = validator;

        match v {
            PathErrorKind::Directory => if !self.is_dir() || !self.exists() {
                events.push(Event::with_error(s, "not a valid directory", Error::InvalidDirectory(self.clone())));
            },
            PathErrorKind::FileExists => if !self.is_file() || !self.exists() {
                events.push(Event::with_error(s, "file does not exists", Error::FileNotFound(self.clone())));
            }
            PathErrorKind::FilePath => if !self.is_file() {
                events.push(Event::with_error(s, "not a valid file path", Error::InvalidFilePath(self.clone())));
            }
        }

        events
    }
}

impl<V, T> Validate<V> for Option<T>
    where
        T: Validate<V>
{
    fn validate(&self, validator: V) -> Vec<Event> {
        if let Some(ref some) = self {
            some.validate(validator)
        } else {
            Vec::new()
        }
    }
}