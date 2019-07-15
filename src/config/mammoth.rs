use std::path::{Path, PathBuf};

use crate::error::event::Event;
use crate::error::severity::Severity;
use crate::error::validate::{Validate, PathErrorKind, PathValidator};

// TODO: Add documentation.
// TODO: Are unit tests needed here?

#[derive(Clone, Debug, Deserialize)]
pub struct Mammoth {
    mods_dir: Option<PathBuf>,
    log_file: Option<PathBuf>,
    log_severity: Option<Severity>
}

impl Mammoth {
    pub fn new() -> Mammoth {
        Mammoth {
            mods_dir: None,
            log_file: None,
            log_severity: None
        }
    }

    pub fn mods_dir(&self) -> Option<&Path> {
        if let Some(ref path) = self.mods_dir { Some(path.as_path()) }
        else { None }
    }

    pub fn log_file(&self) -> Option<&Path> {
        if let Some(ref path) = self.log_file { Some(path.as_path()) }
        else { None }
    }

    pub fn log_severity(&self) -> Option<Severity> {
        self.log_severity
    }

    pub fn set_mods_dir<P>(&mut self, path: P)
        where
            P: AsRef<Path>
    {
        self.mods_dir = Some(path.as_ref().to_path_buf());
    }

    pub fn set_log_file<P>(&mut self, path: P)
        where
            P: AsRef<Path>
    {
        self.log_file = Some(path.as_ref().to_path_buf());
    }

    pub fn set_log_severity(&mut self, severity: Severity) {
        self.log_severity = Some(severity);
    }
}

impl Validate<()> for Mammoth {
    fn validate(&self, _: ()) -> Vec<Event> {
        let mut events = Vec::new();

        events.append(&mut self.mods_dir.validate(PathValidator(PathErrorKind::Directory, Severity::Critical)));
        events.append(&mut self.log_file.validate(PathValidator(PathErrorKind::FilePath, Severity::Critical)));

        events
    }
}