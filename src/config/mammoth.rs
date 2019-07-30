//! The `Mammoth` structure contains the general configuration for Mammoth, such as the location of
//! the modules and the log settings.
use std::path::{Path, PathBuf};

use crate::error::Error;
use crate::error::severity::Severity;
use crate::diagnostics::Logger;
use crate::diagnostics::{Validator, PathValidator, PathValidatorKind};

/// Structure that defines the general configuration for the Mammoth application.
#[derive(Clone, Debug, Deserialize)]
pub struct Mammoth {
    mods_dir: Option<PathBuf>,
    log_file: Option<PathBuf>,
    log_severity: Option<Severity>
}

impl Mammoth {
    /// Creates a new, empty `Mammoth` structure.
    pub fn new() -> Mammoth {
        Mammoth {
            mods_dir: None,
            log_file: None,
            log_severity: None
        }
    }

    /// Obtains the modules directory.
    pub fn mods_dir(&self) -> Option<&Path> {
        if let Some(ref path) = self.mods_dir { Some(path.as_path()) }
        else { None }
    }
    /// Obtains the log file path.
    pub fn log_file(&self) -> Option<&Path> {
        if let Some(ref path) = self.log_file { Some(path.as_path()) }
        else { None }
    }
    /// Obtains the log severity.
    pub fn log_severity(&self) -> Option<Severity> {
        self.log_severity
    }
    /// Sets the modules directory.
    pub fn set_mods_dir<P>(&mut self, path: P)
        where
            P: AsRef<Path>
    {
        self.mods_dir = Some(path.as_ref().to_path_buf());
    }
    /// Sets the log file path.
    pub fn set_log_file<P>(&mut self, path: P)
        where
            P: AsRef<Path>
    {
        self.log_file = Some(path.as_ref().to_path_buf());
    }
    /// Sets the log severity.
    pub fn set_log_severity(&mut self, severity: Severity) {
        self.log_severity = Some(severity);
    }
}

impl Validator<Mammoth> for () {
    fn validate(&self, logger: &mut Logger, item: &Mammoth) -> Result<(), Error> {
        if let Some(mods_dir) = item.mods_dir() {
            PathValidator(Severity::Error, PathValidatorKind::ExistingDirectory)
                .validate(logger, &mods_dir)?;
        }
        if let Some(log_file) = item.log_file() {
            PathValidator(Severity::Error, PathValidatorKind::FilePath)
                .validate(logger, &log_file)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use crate::config::Mammoth;
    use crate::error::severity::Severity;

    #[test]
    /// Generic property test.
    fn test_generic() {
        let mut mammoth = Mammoth::new();

        assert!(mammoth.mods_dir().is_none());
        assert!(mammoth.log_file().is_none());
        assert!(mammoth.log_severity().is_none());

        mammoth.set_mods_dir("./mods/");

        assert_eq!(mammoth.mods_dir().unwrap(), Path::new("./mods/"));
        assert!(mammoth.log_file().is_none());
        assert!(mammoth.log_severity().is_none());

        mammoth.set_log_file("mammoth.log");

        assert_eq!(mammoth.mods_dir().unwrap(), Path::new("./mods/"));
        assert_eq!(mammoth.log_file().unwrap(), Path::new("mammoth.log"));
        assert!(mammoth.log_severity().is_none());

        mammoth.set_log_severity(Severity::Warning);

        assert_eq!(mammoth.mods_dir().unwrap(), Path::new("./mods/"));
        assert_eq!(mammoth.log_file().unwrap(), Path::new("mammoth.log"));
        assert_eq!(mammoth.log_severity().unwrap(), Severity::Warning);
    }
}