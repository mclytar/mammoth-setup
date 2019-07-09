//! The `Binding` structure contains the configuration for a binding port.

use std::fmt::Formatter;
use std::path::{Path, PathBuf};

use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use serde::{Deserialize, Deserializer};
use serde::de::{MapAccess, Visitor};

// TODO: Remove `failure` crate dependency.
// TODO: Perhaps add a `validate` function to validate information?

/// Structure that defines configuration for a binding port.
#[derive(Clone, Debug, PartialEq)]
pub struct Binding {
    port: u16,
    secure: bool,
    cert: Option<PathBuf>,
    key: Option<PathBuf>
}

#[doc(hidden)]
#[derive(Deserialize)]
#[serde(field_identifier, rename_all = "lowercase")]
pub(super) enum PortFields {
    Port,
    Secure,
    Cert,
    Key
}

#[doc(hidden)]
struct PortVisitor;

impl Binding {
    /// Creates a new `Binding` structure for a port, given the port number.
    pub fn new(port: u16) -> Binding {
        Binding {
            port,
            secure: false,
            cert: None,
            key: None
        }
    }
    /// Creates a new `Binding` structure for a secure port,
    /// given the port number and the paths to the certificate and the relative key.
    pub fn with_security<P, Q>(port: u16, cert: P, key: Q) -> Binding
        where
            P: AsRef<Path>,
            Q: AsRef<Path> {
        Binding {
            port,
            secure: true,
            cert: Some(cert.as_ref().to_path_buf()),
            key: Some(key.as_ref().to_path_buf())
        }
    }
    /// Obtains the port number.
    pub fn port(&self) -> u16 {
        self.port
    }
    /// Returns a value that indicates if the binding is secure or not.
    pub fn secure(&self) -> bool {
        self.secure
    }
    /// Obtains the path to the certificate file, if any.
    pub fn cert(&self) -> Option<&Path> {
        if let Some(ref path) = self.cert { Some(path) }
        else { None }
    }
    /// Obtains the path to the key file, if any.
    pub fn key(&self) -> Option<&Path> {
        if let Some(ref path) = self.key { Some(path) }
        else { None }
    }
    /// Sets the port number.
    pub fn set_port(&mut self, port: u16) {
        self.port = port;
    }
    /// Removes security from this binding.
    pub fn clear_security(&mut self) {
        self.secure = false;
        self.cert = None;
        self.key = None;
    }
    /// Sets security for this binding, given a path to a certificate and a path to the relative key.
    pub fn set_security<P, Q>(&mut self, cert: P, key: Q)
        where
            P: AsRef<Path>,
            Q: AsRef<Path>
    {
        self.secure = true;
        self.cert = Some(cert.as_ref().to_path_buf());
        self.key = Some(key.as_ref().to_path_buf());
    }
    /// Tries to construct a `SslAcceptor` structure from the given certificate and key files.
    pub fn ssl_acceptor(&self) -> Result<SslAcceptor, failure::Error> {
        if self.secure {
            let mut ssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
            ssl_builder.set_private_key_file(self.key.as_ref().unwrap(), SslFiletype::PEM)?;
            ssl_builder.set_certificate_chain_file(self.cert.as_ref().unwrap())?;

            Ok(ssl_builder.build())
        } else {
            Err(failure::err_msg("Tried to obtain a SslAcceptor from an insecure binding"))
        }
    }
    /// Obtains an address string from the given port.
    pub fn to_addr_string(&self) -> String {
        format!("0.0.0.0:{}", self.port)
    }
    /// Returns a `Result` indicating if the current `Binding` structure is valid.
    pub fn validate(&self) -> Result<(), failure::Error> {
        if self.secure {
            let _ = self.ssl_acceptor()?;
        }

        Ok(())
    }
}

impl From<u16> for Binding {
    fn from(value: u16) -> Self {
        Binding {
            port: value,
            secure: false,
            cert: None,
            key: None
        }
    }
}

impl <'de> Visitor<'de> for PortVisitor {
    type Value = Binding;

    fn expecting(&self, f: &mut Formatter) -> ::std::fmt::Result {
        write!(f, "a positive number less than 65536 or an object containing the binding parameters.")
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E> where
        E: ::std::error::Error, {
        Ok(Binding::from(v as u16))
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E> where
        E: ::std::error::Error, {
        Ok(Binding::from(v as u16))
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E> where
        E: ::std::error::Error, {
        Ok(Binding::from(v as u16))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> where
        E: ::std::error::Error, {
        Ok(Binding::from(v as u16))
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E> where
        E: ::std::error::Error, {
        Ok(Binding::from(v as u16))
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E> where
        E: ::std::error::Error, {
        Ok(Binding::from(v))
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E> where
        E: ::std::error::Error, {
        Ok(Binding::from(v as u16))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> where
        E: ::std::error::Error, {
        Ok(Binding::from(v as u16))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where
        A: MapAccess<'de>, {
        let mut port: Option<u16> = None;
        let mut secure: Option<bool> = None;
        let mut cert: Option<PathBuf> = None;
        let mut key: Option<PathBuf> = None;

        while let Some(k) = map.next_key()? {
            match k {
                PortFields::Port => {
                    if port.is_some() { return Err(serde::de::Error::duplicate_field("port")); }
                    port = Some(map.next_value()?);
                }
                PortFields::Secure => {
                    if secure.is_some() { return Err(serde::de::Error::duplicate_field("secure")); }
                    secure = Some(map.next_value()?);
                }
                PortFields::Cert => {
                    if cert.is_some() { return Err(serde::de::Error::duplicate_field("cert")); }
                    cert = Some(map.next_value()?);
                }
                PortFields::Key => {
                    if key.is_some() { return Err(serde::de::Error::duplicate_field("key")); }
                    key = Some(map.next_value()?);
                }
            }
        }

        let port = port.ok_or_else(|| serde::de::Error::missing_field("port"))?;
        if let Some(false) = secure {
            Ok(Binding::new(port))
        } else if secure.unwrap_or(false) || cert.is_some() || key.is_some() {
            if cert.is_none() { return Err(serde::de::Error::missing_field("cert")); }
            if key.is_none() { return Err(serde::de::Error::missing_field("key")); }

            Ok(Binding::with_security(port, cert.unwrap(), key.unwrap()))
        } else {
            Ok(Binding::new(port))
        }
    }
}

impl<'de> Deserialize<'de> for Binding {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
        D: Deserializer<'de> {
        deserializer.deserialize_any(PortVisitor)
    }
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;
    use std::path::Path;

    use super::Binding;

    #[test]
    /// Tests parameters handling.
    fn test_parameters() {
        let mut param = Binding::new(80);

        param.set_port(8080);

        assert_eq!(param.port(), 8080);
        assert_eq!(param.secure(), false);
        assert!(param.cert().is_none());
        assert!(param.key().is_none());

        param.set_security("./cert.pem", "./key.pem");

        assert_eq!(param.port(), 8080);
        assert_eq!(param.secure(), true);
        assert_eq!(param.cert().unwrap(), Path::new("./cert.pem"));
        assert_eq!(param.key().unwrap(), Path::new("./key.pem"));

        param.set_port(8443);

        assert_eq!(param.port(), 8443);
        assert_eq!(param.secure(), true);
        assert_eq!(param.cert().unwrap(), Path::new("./cert.pem"));
        assert_eq!(param.key().unwrap(), Path::new("./key.pem"));

        param.clear_security();

        assert_eq!(param.port(), 8443);
        assert_eq!(param.secure(), false);
        assert!(param.cert().is_none());
        assert!(param.key().is_none());
    }

    #[test]
    /// Tests `Binding` creation.
    fn test_creation() {
        let param = Binding::new(80);

        assert_eq!(param.port(), 80);
        assert_eq!(param.secure(), false);
        assert!(param.cert().is_none());
        assert!(param.key().is_none());

        let param_sec = Binding::with_security(443, "./cert.pem", "./key.pem");

        assert_eq!(param_sec.port(), 443);
        assert_eq!(param_sec.secure(), true);
        assert_eq!(param_sec.cert().unwrap(), Path::new("./cert.pem"));
        assert_eq!(param_sec.key().unwrap(), Path::new("./key.pem"));
    }

    #[test]
    /// Tests the `From<u16>` trait implementation.
    fn test_from() {
        assert_eq!(Binding::from(80), Binding::new(80));
    }

    #[test]
    /// Tests deserialization from port number.
    fn test_deserialize_u16() {
        let toml = r#"
        port = 80
        "#;

        let param = toml::from_str::<BTreeMap<String, Binding>>(toml).unwrap().get("port").unwrap().to_owned();
        let test = Binding::new(80);

        assert_eq!(param, test);
    }

    #[test]
    /// Tests deserialization from map.
    fn test_deserialize_map() {
        let toml = r#"
        port = 443
        secure = true
        cert = "./cert.pem"
        key = "./key.pem"
        "#;

        let param = toml::from_str::<Binding>(toml).unwrap();
        let test = Binding::with_security(443, "./cert.pem", "./key.pem");

        assert_eq!(param, test);
    }

    #[test]
    /// Tests deserialization from map, when the map contains only the port number.
    fn test_deserialize_map_autodetect_secure_false() {
        let toml = r#"
        port = 80
        "#;

        let param = toml::from_str::<Binding>(toml).unwrap();
        let test = Binding::new(80);

        assert_eq!(param, test);
    }

    #[test]
    /// Tests deserialization from map, when the map contains the certificate and key paths
    /// but not the `enabled` flag.
    fn test_deserialize_map_autodetect_secure_true() {
        let toml = r#"
        port = 443
        cert = "./cert.pem"
        key = "./key.pem"
        "#;

        let param = toml::from_str::<Binding>(toml).unwrap();
        let test = Binding::with_security(443, "./cert.pem", "./key.pem");

        assert_eq!(param, test);
    }

    #[test]
    /// Tests deserialization from map, when the map contains the certificate and key paths
    /// but the `enabled` flag is set to `false`.
    fn test_deserialize_map_force_secure_false() {
        let toml = r#"
        port = 443
        secure = false
        cert = "./cert.pem"
        key = "./key.pem"
        "#;

        let param = toml::from_str::<Binding>(toml).unwrap();
        let test = Binding::new(443);

        assert_eq!(param, test);
    }

    #[test]
    /// Tests deserialization errors, i.e. when some data is missing.
    fn test_deserialize_map_error() {
        let toml = r#"
        port = 443
        key = "./key.pem"
        "#;
        assert!(toml::from_str::<Binding>(toml).is_err());
        let toml = r#"
        port = 443
        cert = "./cert.pem"
        "#;
        assert!(toml::from_str::<Binding>(toml).is_err());
        let toml = r#"
        port = 443
        secure = true
        "#;
        assert!(toml::from_str::<Binding>(toml).is_err());
    }

    #[test]
    /// Tests binding string creation.
    fn test_to_addr_string() {
        let param = Binding::new(80);
        let param_sec = Binding::with_security(443, "./cert.pem", "./key.pem");

        assert_eq!(param.to_addr_string(), "0.0.0.0:80");
        assert_eq!(param_sec.to_addr_string(), "0.0.0.0:443");
    }

    // TODO: ssl_acceptor is still untested.
}