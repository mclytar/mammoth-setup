use semver::{Version, VersionReq};

// FOR_LATER: find a better way to make compatibility check.
pub const COMPATIBILITY_STRING: &str = "~0.0.1";

pub fn compatible(version: &Version) -> bool {
    let req = VersionReq::parse(COMPATIBILITY_STRING).unwrap();
    req.matches(version)
}