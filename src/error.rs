pub mod event;
pub mod severity;
//pub mod validate;

use std::error::Error as ErrorTrait;
use std::fmt::{Display, Formatter};
use std::io::Error as IoError;
use std::path::PathBuf;

use openssl::error::ErrorStack as SslError;
use semver::{Version, VersionReq};

#[derive(Debug)]
pub enum Error {
    DuplicateItem(String),
    FileNotFound(PathBuf),
    Generic(Box<ErrorTrait + Send + Sync>),
    InvalidDirectory(PathBuf),
    InvalidFilePath(PathBuf),
    InvalidHostname(String),
    InvalidModuleVersion(Version, VersionReq),
    Io(IoError),
    NoHost,
    NoModsDir,
    SecureBindOnInsecure,
    Ssl(SslError),
    Unknown,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match &self {
            Error::DuplicateItem(name) => write!(f, "Duplicate item: '{}'", name),
            Error::FileNotFound(filename) => write!(f, "File not found: '{}'", filename.to_str().unwrap_or("")),
            Error::Generic(err) => write!(f, "Generic error: {}", err.as_ref()),
            Error::Io(err) => write!(f, "I/O error: {}", err),
            Error::InvalidDirectory(dir) => write!(f, "Invalid directory: '{}'", dir.to_str().unwrap_or("")),
            Error::InvalidFilePath(path) => write!(f, "Invalid path: '{}'", path.to_str().unwrap_or("")),
            Error::InvalidHostname(hostname) => write!(f, "Invalid hostname: '{}'", hostname),
            Error::InvalidModuleVersion(ver, ver_req) => write!(f, "Invalid module version: {}; expected: {}.", ver, ver_req),
            Error::NoHost => write!(f, "No host specified; one required."),
            Error::NoModsDir => write!(f, "No directory specified for modules; required if modules are enabled."),
            Error::SecureBindOnInsecure => write!(f, "Tried to bind to a secure port without a certificate"),
            Error::Ssl(stack) => write!(f, "SSL error: {}", stack),
            Error::Unknown => write!(f, "Unknown"),
        }
    }
}

impl ErrorTrait for Error {
    fn description(&self) -> &str {
        match &self {
            Error::DuplicateItem(_) => "duplicate item",
            Error::FileNotFound(_) => "file not found",
            Error::Generic(_) => "generic error",
            Error::Io(_) => "i/o error",
            Error::InvalidDirectory(_) => "invalid directory",
            Error::InvalidFilePath(_) => "invalid file path",
            Error::InvalidHostname(_) => "invalid hostname",
            Error::InvalidModuleVersion(_, _) => "invalid module version",
            Error::NoHost => "no host",
            Error::NoModsDir => "no mods_dir",
            Error::SecureBindOnInsecure => "secure binding without certificate",
            Error::Ssl(_) => "ssl error",
            Error::Unknown => "unknown"
        }
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Self {
        Error::Io(err)
    }
}

impl From<SslError> for Error {
    fn from(err: SslError) -> Self {
        Error::Ssl(err)
    }
}