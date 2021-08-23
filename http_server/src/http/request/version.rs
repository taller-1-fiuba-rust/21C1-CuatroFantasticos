/// Version
/// Enum that categorizes different Http versions
/// Possible values: V1_1, V2_0, Uninitialized

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}

impl From<&str> for Version {
    fn from(s: &str) -> Version {
        match s {
            "HTTP/1.1" => Version::V1_1,
            _ => Version::Uninitialized,
        }
    }
}

#[test]
fn test_version_into() {
    let m: Version = "HTTP/1.1".into();
    assert_eq!(m, Version::V1_1);
}
