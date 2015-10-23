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
    ///     assert_eq!(slice.get_section_items(Kind::Os), Some(vec!["debian"]));
    ///     assert_eq!(slice.get_section_items(Kind::Dep), None);
    /// }
    /// ```
    pub fn get_section_items(&self, section_kind: Kind) -> Option<Vec<&str>> {
        let mut iter = self.sections.iter();
        let section = iter.find(|section| section.kind == section_kind);
        if let Some(section) = section {
            let iter = section.items.iter();
            let items = iter.map(|item| item.deref()).collect::<Vec<&str>>();
            Some(items)
        } else {
            None
        }
    }

    pub fn dependencies(&self) -> Vec<&str> {
        if let Some(dependencies) = self.get_section_items(Kind::Dep) {
            dependencies
        } else {
            Vec::new()
        }
    }

    pub fn supports_os(&self, os: &str) -> bool {
        self.get_os_list().contains(&os)
    }

    pub fn get_os_list(&self) -> Vec<&str> {
        if let Some(os_list) = self.get_section_items(Kind::Os) {
            os_list
        } else {
            panic!("Slice has no os section. Slice path = {}", self.path.display());
        }
    }

    pub fn run_section(&self) -> Vec<&str> {
        if let Some(run_section) = self.get_section_items(Kind::Run) {
            run_section
        } else {
            panic!("Slice has no run section. Slice path = {}", self.path.display())
        }
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
    let os_list = slice.get_section_items(Kind::Os).unwrap();
    assert_eq!(os_list.len(), 1);
    let os = *os_list.first().unwrap();
    assert_eq!(os, "debian-8.2");
}
