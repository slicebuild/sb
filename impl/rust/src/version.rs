use std::str::FromStr;
use semver::Version;

pub fn parse(str: &str) -> Version {
    match Version::parse(str) {
        Ok(version) => version,
        Err(_) => parse_invalid_version(str)
    }
}

pub fn zero() -> Version {
    Version { major: 0, minor: 0, patch: 0, pre: Vec::new(), build: Vec::new() }
}

fn parse_invalid_version(str: &str) -> Version {
    if str.is_empty() {
        return zero();
    }
    match str.to_string().chars().filter(|&c| c == '.').count() {
        0 => Version { major: u64::from_str(str).unwrap(), .. zero() },
        1 => {
            let parts = str.split('.').collect::<Vec<_>>();
            let major = u64::from_str(parts[0]).unwrap();
            let minor = u64::from_str(parts[1]).unwrap();
            Version { major : major, minor: minor, .. zero() }
        },
        _ => panic!("Invalid version string = {}", str)
    }
}

#[cfg(test)]
mod tests {
    use semver::{Identifier, Version};

    #[test]
    fn only_major() {
        let version = Version { major: 1, minor: 0, patch: 0, pre: Vec::new(),
                                build: Vec::new() };
        assert_eq!(super::parse("1"), version);
    }

    #[test]
    fn major_and_minor() {
        let version = Version { major: 1, minor: 2, patch: 0, pre: Vec::new(),
                                build: Vec::new() };
        assert_eq!(super::parse("1.2"), version);
    }

    #[test]
    fn major_and_minor_and_patch() {
        let version = Version { major: 1, minor: 2, patch: 3, pre: Vec::new(),
                                build: Vec::new() };
        assert_eq!(super::parse("1.2.3"), version);
    }

    #[test]
    fn major_and_minor_and_patch_and_prerelease() {
        let version = Version { major: 1, minor: 2, patch: 3,
                                pre: vec![Identifier::AlphaNumeric("alpha".to_string())],
                                build: Vec::new() };
        assert_eq!(super::parse("1.2.3-alpha"), version);
    }

    #[test]
    fn major_and_minor_and_patch_and_prerelease_and_build() {
        let version = Version { major: 1, minor: 2, patch: 3,
                                pre: vec![Identifier::AlphaNumeric("alpha".to_string()),
                                          Identifier::Numeric(5)],
                                build: Vec::new() };
        assert_eq!(super::parse("1.2.3-alpha.5"), version);
    }
}