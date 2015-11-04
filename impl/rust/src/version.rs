use std::str::FromStr;
use semver::Version;

pub fn parse(str: &str) -> Version {
    match Version::parse(str) {
        Ok(version) => version,
        Err(_) => parse_invalid_version(str)
    }
}

pub fn extract_name_and_version(string: &str) -> (String, Version) {
    assert_not_empty!(string);
    let iter = string.chars().enumerate();
    let positions = iter.filter(|&(_, c)| c == '-')
                        .map(|(i, _)| i)
                        .filter(|i| {
        if let Some(char) = string.chars().nth(i + 1) {
            char.is_digit(10)
        } else {
            false
        }
                        })
                        .collect::<Vec<_>>();
    match positions.len() {
        0 => (string.to_string(), zero()),
        _ => {
            let pos = *positions.last().unwrap();
            (string[..pos].to_string(), parse(&string[pos + 1..]))
        }
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
    fn slice_with_only_major() {
        let (name, version) = super::extract_name_and_version("apache-2");
        assert_eq!(name, "apache");
        assert_eq!(version, super::parse("2"));
    }

    #[test]
    fn slice_with_dash_in_name() {
        let (name, version) = super::extract_name_and_version("my-apache-2");
        assert_eq!(name, "my-apache");
        assert_eq!(version, super::parse("2"));
    }

    #[test]
    fn get_slice_name_and_version_from_string() {
        let string = "my_app-2.0.0-beta".to_string();
        let (slice_name, version) = super::extract_name_and_version(&string);
        assert_eq!(slice_name, "my_app");
        assert_eq!(version, super::parse("2.0.0-beta"));
    }

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