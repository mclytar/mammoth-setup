use std::fmt;
use std::fmt::{Display, Formatter};

use serde::de::{Deserialize, Deserializer, Error, Unexpected, Visitor};
use serde::ser::{Serialize, Serializer};

/// Describes the severity of the Log report.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Severity {
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
    type Value = Severity;

    fn expecting(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, r#""debug", "information", "warning", "error" or "critical""#)
    }

    fn visit_str<E>(self, v: &str) -> Result<Severity, E> where
        E: Error {
        let code_str = v.to_lowercase();

        match &code_str[..] {
            "debug" => Ok(Severity::Debug),
            "information" => Ok(Severity::Information),
            "warning" => Ok(Severity::Warning),
            "error" => Ok(Severity::Error),
            "critical" => Ok(Severity::Critical),
            _ => Err(Error::invalid_value(Unexpected::Str(&code_str), &self))
        }
    }
}

impl<'de> Deserialize<'de> for Severity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
        D: Deserializer<'de> {
        deserializer.deserialize_str(SeverityVisitor)
    }
}

impl Serialize for Severity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
        S: Serializer {
        match &self {
            Severity::Debug => serializer.serialize_str("debug"),
            Severity::Information => serializer.serialize_str("information"),
            Severity::Warning => serializer.serialize_str("warning"),
            Severity::Error => serializer.serialize_str("error"),
            Severity::Critical => serializer.serialize_str("critical")
        }
    }
}

impl Display for Severity {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match &self {
            Severity::Debug => write!(f, "DBG "),
            Severity::Information => write!(f, "INFO"),
            Severity::Warning => write!(f, "WARN"),
            Severity::Error => write!(f, "ERR "),
            Severity::Critical => writeln!(f, "CRIT")
        }
    }
}

impl Default for Severity {
    fn default() -> Self {
        Severity::Error
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::BTreeMap;

    #[test]
    /// Tests deserialization of `debug` variant.
    fn test_deserialize_debug() {
        assert_eq!(toml::from_str::<BTreeMap<String, Severity>>(r#"sr = "debug""#).unwrap().get("sr").unwrap().to_owned(), Severity::Debug);
        assert_eq!(toml::from_str::<BTreeMap<String, Severity>>(r#"sr = "Debug""#).unwrap().get("sr").unwrap().to_owned(), Severity::Debug);
        assert_eq!(toml::from_str::<BTreeMap<String, Severity>>(r#"sr = "DEBUG""#).unwrap().get("sr").unwrap().to_owned(), Severity::Debug);
    }

    #[test]
    /// Tests deserialization of `information` variant.
    fn test_deserialize_information() {
        assert_eq!(toml::from_str::<BTreeMap<String, Severity>>(r#"sr = "information""#).unwrap().get("sr").unwrap().to_owned(), Severity::Information);
        assert_eq!(toml::from_str::<BTreeMap<String, Severity>>(r#"sr = "Information""#).unwrap().get("sr").unwrap().to_owned(), Severity::Information);
        assert_eq!(toml::from_str::<BTreeMap<String, Severity>>(r#"sr = "INFORMATION""#).unwrap().get("sr").unwrap().to_owned(), Severity::Information);
    }

    #[test]
    /// Tests deserialization of `warning` variant.
    fn test_deserialize_warning() {
        assert_eq!(toml::from_str::<BTreeMap<String, Severity>>(r#"sr = "warning""#).unwrap().get("sr").unwrap().to_owned(), Severity::Warning);
        assert_eq!(toml::from_str::<BTreeMap<String, Severity>>(r#"sr = "Warning""#).unwrap().get("sr").unwrap().to_owned(), Severity::Warning);
        assert_eq!(toml::from_str::<BTreeMap<String, Severity>>(r#"sr = "WARNING""#).unwrap().get("sr").unwrap().to_owned(), Severity::Warning);
    }

    #[test]
    /// Tests deserialization of `error` variant.
    fn test_deserialize_error() {
        assert_eq!(toml::from_str::<BTreeMap<String, Severity>>(r#"sr = "error""#).unwrap().get("sr").unwrap().to_owned(), Severity::Error);
        assert_eq!(toml::from_str::<BTreeMap<String, Severity>>(r#"sr = "Error""#).unwrap().get("sr").unwrap().to_owned(), Severity::Error);
        assert_eq!(toml::from_str::<BTreeMap<String, Severity>>(r#"sr = "ERROR""#).unwrap().get("sr").unwrap().to_owned(), Severity::Error);
    }

    #[test]
    /// Tests deserialization of `critical` variant.
    fn test_deserialize_critical() {
        assert_eq!(toml::from_str::<BTreeMap<String, Severity>>(r#"sr = "critical""#).unwrap().get("sr").unwrap().to_owned(), Severity::Critical);
        assert_eq!(toml::from_str::<BTreeMap<String, Severity>>(r#"sr = "Critical""#).unwrap().get("sr").unwrap().to_owned(), Severity::Critical);
        assert_eq!(toml::from_str::<BTreeMap<String, Severity>>(r#"sr = "CRITICAL""#).unwrap().get("sr").unwrap().to_owned(), Severity::Critical);
    }

    #[test]
    #[should_panic]
    /// Tests deserialization of an invalid variant.
    fn test_deserialize_invalid() {
        let _ = toml::from_str::<BTreeMap<String, Severity>>(r#"sr = "dummy""#).unwrap();
    }
}