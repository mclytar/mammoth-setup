use std::fmt;
use std::fmt::Formatter;

use serde::de::{Deserialize, Deserializer, Error, Unexpected, Visitor};
use serde::ser::{Serialize, Serializer};

/// Describes the severity of the Log report.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LogSeverity {
    /// The log should output every useful and technical information.
    ///
    /// **Note**: this configuration should NOT be used in a production server due to the information
    /// content possibly being sensible.
    Debug,
    /// The log should output every useful information, but can omit information that is too
    /// technical. Sensible information should be avoided.
    Information,
    /// The log should output only information about possibly problematic or unexpected situations.
    Warning,
    /// The log should output only information about execution-breaking situations.
    Error,
    /// The log should output only information about application-breaking situations (i.e. when
    /// the application encounters an unrecoverable error and must exit with some error status).
    Critical
}

/// Case-insensitive visitor for `SeverityReport` deserialization.
struct SeverityVisitor;

impl<'de> Visitor<'de> for SeverityVisitor {
    type Value = LogSeverity;

    fn expecting(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, r#""debug", "information", "warning", "error" or "critical""#)
    }

    fn visit_str<E>(self, v: &str) -> Result<LogSeverity, E> where
        E: Error {
        let code_str = v.to_lowercase();

        match &code_str[..] {
            "debug" => Ok(LogSeverity::Debug),
            "information" => Ok(LogSeverity::Information),
            "warning" => Ok(LogSeverity::Warning),
            "error" => Ok(LogSeverity::Error),
            "critical" => Ok(LogSeverity::Critical),
            _ => Err(Error::invalid_value(Unexpected::Str(&code_str), &self))
        }
    }
}

impl<'de> Deserialize<'de> for LogSeverity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
        D: Deserializer<'de> {
        deserializer.deserialize_str(SeverityVisitor)
    }
}

impl Serialize for LogSeverity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
        S: Serializer {
        match &self {
            LogSeverity::Debug => serializer.serialize_str("debug"),
            LogSeverity::Information => serializer.serialize_str("information"),
            LogSeverity::Warning => serializer.serialize_str("warning"),
            LogSeverity::Error => serializer.serialize_str("error"),
            LogSeverity::Critical => serializer.serialize_str("critical")
        }
    }
}

impl Default for LogSeverity {
    fn default() -> Self {
        LogSeverity::Error
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::BTreeMap;

    #[test]
    /// Tests deserialization of `debug` variant.
    fn test_deserialize_debug() {
        assert_eq!(toml::from_str::<BTreeMap<String, LogSeverity>>(r#"sr = "debug""#).unwrap().get("sr").unwrap().to_owned(), LogSeverity::Debug);
        assert_eq!(toml::from_str::<BTreeMap<String, LogSeverity>>(r#"sr = "Debug""#).unwrap().get("sr").unwrap().to_owned(), LogSeverity::Debug);
        assert_eq!(toml::from_str::<BTreeMap<String, LogSeverity>>(r#"sr = "DEBUG""#).unwrap().get("sr").unwrap().to_owned(), LogSeverity::Debug);
    }

    #[test]
    /// Tests deserialization of `information` variant.
    fn test_deserialize_information() {
        assert_eq!(toml::from_str::<BTreeMap<String, LogSeverity>>(r#"sr = "information""#).unwrap().get("sr").unwrap().to_owned(), LogSeverity::Information);
        assert_eq!(toml::from_str::<BTreeMap<String, LogSeverity>>(r#"sr = "Information""#).unwrap().get("sr").unwrap().to_owned(), LogSeverity::Information);
        assert_eq!(toml::from_str::<BTreeMap<String, LogSeverity>>(r#"sr = "INFORMATION""#).unwrap().get("sr").unwrap().to_owned(), LogSeverity::Information);
    }

    #[test]
    /// Tests deserialization of `warning` variant.
    fn test_deserialize_warning() {
        assert_eq!(toml::from_str::<BTreeMap<String, LogSeverity>>(r#"sr = "warning""#).unwrap().get("sr").unwrap().to_owned(), LogSeverity::Warning);
        assert_eq!(toml::from_str::<BTreeMap<String, LogSeverity>>(r#"sr = "Warning""#).unwrap().get("sr").unwrap().to_owned(), LogSeverity::Warning);
        assert_eq!(toml::from_str::<BTreeMap<String, LogSeverity>>(r#"sr = "WARNING""#).unwrap().get("sr").unwrap().to_owned(), LogSeverity::Warning);
    }

    #[test]
    /// Tests deserialization of `error` variant.
    fn test_deserialize_error() {
        assert_eq!(toml::from_str::<BTreeMap<String, LogSeverity>>(r#"sr = "error""#).unwrap().get("sr").unwrap().to_owned(), LogSeverity::Error);
        assert_eq!(toml::from_str::<BTreeMap<String, LogSeverity>>(r#"sr = "Error""#).unwrap().get("sr").unwrap().to_owned(), LogSeverity::Error);
        assert_eq!(toml::from_str::<BTreeMap<String, LogSeverity>>(r#"sr = "ERROR""#).unwrap().get("sr").unwrap().to_owned(), LogSeverity::Error);
    }

    #[test]
    /// Tests deserialization of `critical` variant.
    fn test_deserialize_critical() {
        assert_eq!(toml::from_str::<BTreeMap<String, LogSeverity>>(r#"sr = "critical""#).unwrap().get("sr").unwrap().to_owned(), LogSeverity::Critical);
        assert_eq!(toml::from_str::<BTreeMap<String, LogSeverity>>(r#"sr = "Critical""#).unwrap().get("sr").unwrap().to_owned(), LogSeverity::Critical);
        assert_eq!(toml::from_str::<BTreeMap<String, LogSeverity>>(r#"sr = "CRITICAL""#).unwrap().get("sr").unwrap().to_owned(), LogSeverity::Critical);
    }

    #[test]
    #[should_panic]
    /// Tests deserialization of an invalid variant.
    fn test_deserialize_invalid() {
        let _ = toml::from_str::<BTreeMap<String, LogSeverity>>(r#"sr = "dummy""#).unwrap();
    }
}