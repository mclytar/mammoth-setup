use std::error::Error as ErrorTrait;
use std::fmt::{Display, Formatter};

use chrono::{DateTime, Local};

use crate::error::severity::Severity;
use super::Error;

pub fn debug(description: &str) -> Event {
    Event {
        timestamp: Local::now(),
        description: description.to_owned(),
        error: None,
        severity: Severity::Debug
    }
}
pub fn info(description: &str) -> Event {
    Event {
        timestamp: Local::now(),
        description: description.to_owned(),
        error: None,
        severity: Severity::Information
    }
}
pub fn warn(description: &str) -> Event {
    Event {
        timestamp: Local::now(),
        description: description.to_owned(),
        error: None,
        severity: Severity::Warning
    }
}
pub fn err(description: &str) -> Event {
    Event {
        timestamp: Local::now(),
        description: description.to_owned(),
        error: None,
        severity: Severity::Error
    }
}
pub fn critical(description: &str) -> Event {
    Event {
        timestamp: Local::now(),
        description: description.to_owned(),
        error: None,
        severity: Severity::Critical
    }
}
pub fn debug_error(description: &str, err: Error) -> Event {
    Event {
        timestamp: Local::now(),
        description: description.to_owned(),
        error: Some(err),
        severity: Severity::Debug
    }
}
pub fn info_error(description: &str, err: Error) -> Event {
    Event {
        timestamp: Local::now(),
        description: description.to_owned(),
        error: Some(err),
        severity: Severity::Information
    }
}
pub fn warn_error(description: &str, err: Error) -> Event {
    Event {
        timestamp: Local::now(),
        description: description.to_owned(),
        error: Some(err),
        severity: Severity::Warning
    }
}
pub fn err_error(description: &str, err: Error) -> Event {
    Event {
        timestamp: Local::now(),
        description: description.to_owned(),
        error: Some(err),
        severity: Severity::Error
    }
}
pub fn critical_error(description: &str, err: Error) -> Event {
    Event {
        timestamp: Local::now(),
        description: description.to_owned(),
        error: Some(err),
        severity: Severity::Critical
    }
}

#[derive(Debug)]
pub struct Event {
    pub(in self) timestamp: DateTime<Local>,
    pub(in self) description: String,
    pub(in self) error: Option<Error>,
    pub(in self) severity: Severity
}

impl Event {
    pub fn new(severity: Severity, description: &str) -> Event {
        Event {
            timestamp: Local::now(),
            description: description.to_owned(),
            error: None,
            severity
        }
    }
    pub fn with_error(severity: Severity, description: &str, error: Error) -> Event {
        Event {
            timestamp: Local::now(),
            description: description.to_owned(),
            error: Some(error),
            severity
        }
    }
}

impl Display for Event {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{} [{}]: {}\n", self.timestamp, self.severity, self.description)
    }
}

impl ErrorTrait for Event {
    fn description(&self) -> &str {
        &self.description
    }
}