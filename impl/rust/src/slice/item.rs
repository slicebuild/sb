use std::cmp::Ordering;
use std::ops::Deref;
use std::path::PathBuf;
use semver::Version;
use super::section::{Kind, Section};

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct Slice {
    pub name: String,
    pub path: PathBuf,
    pub version: Version,
    pub sections: Vec<Section>,
}

impl Slice {
    /// # Examples
    /// ```
    /// extern crate semver;
    /// extern crate sb;
    /// use std::path::PathBuf;
    /// use semver::Version;
    /// use sb::slice::section::{Section, Kind};
    /// use sb::slice::item::Slice;
    ///
    /// fn main() {
    ///     let section = Section { kind: Kind::Os, items: vec!["debian".to_string()] };
    ///     let sections = vec![section];
    ///     let slice = Slice { name: "".to_string(), path: PathBuf::new(),
    ///                         version: Version::parse("0.0.0").unwrap(),
    ///                         sections: sections };
    ///     assert_eq!(slice.section(Kind::Os), Some(vec!["debian"]));
    ///     assert_eq!(slice.section(Kind::Dep), None);
    /// }
    /// ```
    pub fn section(&self, kind: Kind) -> Vec<&str> {
        let mut iter = self.sections.iter();
        let section = iter.find(|section| section.kind == kind);
        if let Some(section) = section {
            let iter = section.items.iter();
            let items = iter.map(|item| item.deref()).collect::<Vec<&str>>();
            items
        } else {
            Vec::new()
        }
    }

    pub fn dependencies(&self) -> Vec<&str> {
        self.section(Kind::Dep)
    }

    pub fn supports_os(&self, os: &str) -> bool {
        self.oses().contains(&os)
    }

    pub fn oses(&self) -> Vec<&str> {
        let section = self.section(Kind::Os);
        assert!(!section.is_empty(), "Slice has no os section. Slice path = {}",
                self.path.display());
        section
    }

    pub fn run_section(&self) -> Vec<&str> {
        let section = self.section(Kind::Run);
        assert!(!section.is_empty(), "Slice has no run section. Slice path = {}",
                self.path.display());
        section
    }
}

impl PartialOrd for Slice {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.version.partial_cmp(&other.version)
    }
}

impl Eq for Slice {
}

impl Ord for Slice {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[test]
fn slice_get_os_list_works() {
    let mut lines: Vec<String> = Vec::new();
    lines.push("OS".to_string());
    lines.push("debian-8.2".to_string());
    lines.push("".to_string());
    lines.push("FROM".to_string());
    lines.push("debian:jessie".to_string());
    let mut sections: Vec<Section> = Vec::new();
    while !lines.is_empty() {
        let (section, remaining_lines) = Section::load_from_lines(lines);
        let section = section.unwrap();
        sections.push(section);
        lines = remaining_lines;
    }
    assert_eq!(sections.len(), 2);
    let slice = Slice {
        name: "Slice".to_string(),
        path: PathBuf::new(),
        version: Version::parse("0.0.0").unwrap(),
        sections: sections,
    };
    let os_list = slice.section(Kind::Os).unwrap();
    assert_eq!(os_list.len(), 1);
    let os = *os_list.first().unwrap();
    assert_eq!(os, "debian-8.2");
}
