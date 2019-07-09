mod log_severity;

use std::path::{Path, PathBuf};

pub use self::log_severity::LogSeverity;

// TODO: Add documentation.
// TODO: Are unit tests needed here?

#[derive(Debug, Deserialize)]
pub struct Mammoth {
    mods_dir: Option<PathBuf>,
    log_file: Option<PathBuf>,
    log_severity: Option<LogSeverity>
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

    pub fn log_severity(&self) -> Option<LogSeverity> {
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

    pub fn set_log_severity(&mut self, severity: LogSeverity) {
        self.log_severity = Some(severity);
    }
}