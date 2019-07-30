//! Diagnostics utilities to validate and keep track of the entire Mammoth system.
//!
//! This module provides the main traits and structures for both validation and log file writing.

use std::any::Any;
use std::fs::File;
use std::io::Write;
use std::marker::PhantomData;
use std::path::Path;
use std::sync::{Arc, RwLock};

use crate::error::Error;
use crate::error::event::Event;
use crate::error::severity::Severity;

/// Same to `Arc<RwLock<Logger>>`.
pub type AsyncLoggerReference = Arc<RwLock<Logger>>;
/// Same to `Result<(), mammoth_setup::error::Error>`.
pub type ValidationResult = Result<(), Error>;

/// Uniquely identifies something in a collection.
///
/// Whenever a structure needs to have unique properties within a collection of such structures,
/// these properties can be cloned into an `Identifier` and the structure can implement the `Id`
/// trait in order to generate the identifier from the structure.
///
/// An error should be emitted whenever two different items in a `Vec<Id>` collection share the
/// same `id()`.
///
/// # Example
/// ```rust
/// use mammoth_setup::diagnostics::Id;
///
/// struct MyStruct {
///     unique_name: String,
///     some_data: u32
/// }
///
/// impl Id for MyStruct {
///     type Identifier = String;
///
///     fn id(&self) -> Self::Identifier {
///         self.unique_name.clone()
///     }
///
///     // Can be used in order to describe the kind of item.
///     fn description(&self) -> &str {
///         "MyStruct"
///     }
/// }
/// ```
pub trait Id {
    /// Type of the item uniquely identifying the implementor.
    ///
    /// Must implement the `Eq` trait in order to make comparisons.
    type Identifier: Eq;

    /// Returns an identifier that (should) uniquely identify the implementor within a collection.
    fn id(&self) -> Self::Identifier;
    /// Returns a description of the implementor.
    ///
    /// The default behavior is returning the string `item`.
    fn description(&self) -> &str {
        "item"
    }
}

/// Stores information about the execution.
///
/// Can be a vector of events, a file, the standard output or whatever can display or store
/// information.
pub trait Logger: Any + Send + Sync {
    /// Stores a particular information about the execution, along with its severity.
    ///
    /// The `Severity` parameter can be used to exclude some of the information: if a logger keeps
    /// track of the events that have `Severity` greater than or equal to `Warning`, every
    /// information of kind `Debug` or `Information` may be omitted.
    fn log(&mut self, _: Severity, _: &str);
}

impl Logger for Vec<Event> {
    fn log(&mut self, sev: Severity, desc: &str) {
        self.push(Event::new(sev, desc));
    }
}

/// Can produce information about the execution.
///
/// The implementor receives a reference to a `Logger` (more in detail, an `AsyncLoggerReference`,
/// a.k.a. `Arc<RwLock<Logger>>`) and stores it somewhere.
/// Whenever something that should be notified happens (e.g. an error or a debug information), the
/// implementor locks the logger for write and writes in it such information.
pub trait Log
{
    /// Stores the (asynchronous reference to the) logger for later use.
    fn register_logger(&mut self, logger: AsyncLoggerReference);
    /// Retrieves the (asynchronous reference to the) logger, if any.
    fn retrieve_logger(&self) -> Option<AsyncLoggerReference>;
    /// Stores some information in the previously stored logger.
    ///
    /// The default behavior is to gain write access to the logger and store the specified
    /// information.
    fn log(&self, sev: Severity, desc: &str) {
        if let Some(logger) = self.retrieve_logger() {
            let mut alr = logger.write().unwrap();

            alr.log(sev, desc);
        }
    }
}

/// Validates a structure.
///
/// Can be used to check that a configuration structure contains valid data.
pub trait Validator<T> {
    /// Validates an item writing all the validation information into a `Logger`.
    ///
    /// # Returns
    /// An `Error` if the structure contains any error, `Ok` if the structure is valid.
    fn validate(&self, _: &mut Logger, _: &T) -> ValidationResult;
}

impl<T> Validator<T> for Fn(&mut Logger, &T) -> Result<(), Error> {
    fn validate(&self, logger: &mut Logger, item: &T) -> Result<(), Error> {
        self(logger, item)
    }
}

/// Kind of validation for paths.
#[derive(Copy, Clone)]
pub enum PathValidatorKind {
    /// Validates if the path is an existing directory.
    ExistingDirectory,
    /// Validates if the path is an existing file.
    ExistingFile,
    /// Validates if the path is correct for a file name.
    FilePath,
}
/// Validates a path using the specified severity and validator kind.
#[derive(Copy, Clone)]
pub struct PathValidator(pub Severity, pub PathValidatorKind);

impl<P> Validator<P> for PathValidator
    where
        P: AsRef<Path>
{
    fn validate(&self, logger: &mut Logger, item: &P) -> Result<(), Error> {
        let severity = self.0;
        let data = self.1;
        let item = item.as_ref();

        match data {
            PathValidatorKind::FilePath => if item.to_string_lossy().ends_with("/") {
                let desc = format!("Not a valid file name: '{:?}'.", item);
                logger.log(severity, &desc);
                if severity >= Severity::Error { Err(Error::InvalidFilePath(item.to_path_buf()))?; }
            },
            PathValidatorKind::ExistingDirectory => if !item.is_dir() {
                let desc = format!("Directory does not exist: '{:?}'.", item);
                logger.log(severity, &desc);
                if severity >= Severity::Error { Err(Error::FileNotFound(item.to_path_buf()))?; }
            },
            PathValidatorKind::ExistingFile => if !item.is_file() {
                let desc = format!("File does not exist: '{:?}'.", item);
                logger.log(severity, &desc);
                if severity >= Severity::Error { Err(Error::FileNotFound(item.to_path_buf()))?; }
            }
        }

        Ok(())
    }
}
/// Defines an entity (usually, a file) able to collect log information.
///
/// In particular, contains an (asynchronous reference to an) item that implements the `Write` trait
/// in order to write log information.
pub struct LogEntity {
    severity: Severity,
    entity: Arc<RwLock<Write + Send + Sync>>
}

impl LogEntity {
    /// Creates a new `LogEntity` from the specified `severity` and `entity`.
    pub fn new(severity: Severity, entity: Arc<RwLock<Write + Send + Sync>>) -> LogEntity {
        LogEntity {
            severity,
            entity
        }
    }
    /// Creates a new `LogEntity` from the specified `severity` and constructing the relative
    /// log container using the specified file.
    pub fn from_filename<P>(severity: Severity, filename: P) -> Result<LogEntity, Error>
        where
            P: AsRef<Path>
    {
        let file = File::open(filename)?;
        let entity = Arc::new(RwLock::new(file));
        Ok(LogEntity {
            severity,
            entity
        })
    }
}

impl Logger for LogEntity {
    fn log(&mut self, severity: Severity, desc: &str) {
        if severity >= self.severity {
            let datetime = chrono::Local::now();
            let message = format!("{} [{}]: {}\n", datetime.format("%Y-%m-%d %H:%M:%S"), severity, desc);

            let mut writer = self.entity.write().unwrap();
            writer.write_all(message.as_bytes()).unwrap();
        }
    }
}

/// Defines a Validator that validates collections of items implementing the `Id` trait.
///
/// The validator runs the internal validator and, moreover, checks if all the items within a
/// `Vec<impl Id>` have a unique identifier within the vector.
/// If not, the validator emits an error of the specified severity.
pub struct IdValidator<I: Id, V: Validator<I>> (pub Severity, pub V, pub PhantomData<I>);

impl<I, V> Validator<Vec<I>> for IdValidator<I, V>
    where
        I: Id,
        V: Validator<I>
{
    fn validate(&self, logger: &mut Logger, item: &Vec<I>) -> Result<(), Error> {
        let mut uniques = Vec::new();

        for val in item {
            if uniques.contains(&val.id()) || uniques.contains(&val.id()) {
                let desc = format!("Unique item declared twice.");
                logger.log(self.0, &desc);
                Err(Error::DuplicateItem("temp".to_owned()))?;
            } else {
                self.1.validate(logger, val)?;

                uniques.push(val.id());
            }
        }

        Ok(())
    }
}

impl<I, V> Validator<Vec<&I>> for IdValidator<I, V>
    where
        I: Id,
        V: Validator<I>
{
    fn validate(&self, logger: &mut Logger, item: &Vec<&I>) -> Result<(), Error> {
        let mut uniques = Vec::new();

        for &val in item {
            if uniques.contains(&val.id()) || uniques.contains(&val.id()) {
                let desc = format!("Unique item declared twice.");
                logger.log(self.0, &desc);
                Err(Error::DuplicateItem("temp".to_owned()))?;
            } else {
                self.1.validate(logger, val)?;

                uniques.push(val.id());
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Read, Seek, SeekFrom};
    use std::path::Path;
    use std::sync::{Arc, RwLock};

    use crate::diagnostics::{Logger, LogEntity, PathValidator, PathValidatorKind, Validator};
    use crate::error::severity::Severity;
    use crate::error::event::Event;

    #[test]
    /// Tests the `LogEntity` structure using a temporary file.
    fn test_logfile() {
        let file = tempfile::tempfile().unwrap();
        let handler = Arc::new(RwLock::new(file));
        let mut log_file = LogEntity::new(Severity::Warning, handler.clone());

        // check that file is empty.
        {
            let mut result = String::new();
            let mut reader = handler.write().unwrap();
            reader.seek(SeekFrom::Start(0)).unwrap();
            reader.read_to_string(&mut result).unwrap();
            assert_eq!(result, "");
        }
        // write on log.
        {
            log_file.log(Severity::Warning, "Test string.");
            log_file.log(Severity::Error, "Another test string.");
            log_file.log(Severity::Information, "Severity level too low, discard this string.");
        }
        // check that string has been successfully written.
        {
            let datetime = chrono::Local::now();
            let test = format!("{} [WARN]: Test string.\n{} [ERR ]: Another test string.\n", datetime.format("%Y-%m-%d %H:%M:%S"), datetime.format("%Y-%m-%d %H:%M:%S"));
            let mut result = String::new();
            let mut reader = handler.write().unwrap();
            reader.seek(SeekFrom::Start(0)).unwrap();
            reader.read_to_string(&mut result).unwrap();
            assert_eq!(result, test);
        }
    }

    #[test]
    /// Tests the `PathValidator` of kind `ExistingFile`.
    fn test_file_exists_validator() {
        let validator = PathValidator(Severity::Error, PathValidatorKind::ExistingFile);
        let mut events: Vec<Event> = Vec::new();

        assert!(validator.validate(&mut events, &Path::new("i_do_not_exist.txt")).is_err());
        assert!(validator.validate(&mut events, &Path::new("i_do_not_exist.txt/")).is_err());
        assert!(validator.validate(&mut events, &Path::new("Cargo.toml")).is_ok());
        assert!(validator.validate(&mut events, &Path::new("Cargo.toml/")).is_err());
        assert!(validator.validate(&mut events, &Path::new("i_do_not_exist/")).is_err());
        assert!(validator.validate(&mut events, &Path::new("i_do_not_exist")).is_err());
        assert!(validator.validate(&mut events, &Path::new("tests/")).is_err());
        assert!(validator.validate(&mut events, &Path::new("tests")).is_err());
    }

    #[test]
    /// Tests the `PathValidator` of kind `ExistingDirectory`.
    fn test_dir_exists_validator() {
        let validator = PathValidator(Severity::Error, PathValidatorKind::ExistingDirectory);
        let mut events: Vec<Event> = Vec::new();

        assert!(validator.validate(&mut events, &Path::new("i_do_not_exist.txt")).is_err());
        assert!(validator.validate(&mut events, &Path::new("i_do_not_exist.txt/")).is_err());
        assert!(validator.validate(&mut events, &Path::new("Cargo.toml")).is_err());
        assert!(validator.validate(&mut events, &Path::new("Cargo.toml/")).is_err());
        assert!(validator.validate(&mut events, &Path::new("i_do_not_exist/")).is_err());
        assert!(validator.validate(&mut events, &Path::new("i_do_not_exist")).is_err());
        assert!(validator.validate(&mut events, &Path::new("tests/")).is_ok());
        assert!(validator.validate(&mut events, &Path::new("tests")).is_ok());
    }

    #[test]
    /// Tests the `PathValidator` of kind `FilePath`.
    fn test_file_path_validator() {
        let validator = PathValidator(Severity::Error, PathValidatorKind::FilePath);
        let mut events: Vec<Event> = Vec::new();

        assert!(validator.validate(&mut events, &Path::new("i_do_not_exist.txt")).is_ok());
        assert!(validator.validate(&mut events, &Path::new("i_do_not_exist.txt/")).is_err());
        assert!(validator.validate(&mut events, &Path::new("Cargo.toml")).is_ok());
        assert!(validator.validate(&mut events, &Path::new("Cargo.toml/")).is_err());
        assert!(validator.validate(&mut events, &Path::new("i_do_not_exist/")).is_err());
        assert!(validator.validate(&mut events, &Path::new("i_do_not_exist")).is_ok());
        assert!(validator.validate(&mut events, &Path::new("tests/")).is_err());
        assert!(validator.validate(&mut events, &Path::new("tests")).is_ok());
    }
}