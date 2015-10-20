use super::section::{Kind, Section};
use std::ops::Deref;
use semver::Version;

pub struct Slice {
    pub name: String,
    pub version: Option<Version>,
    pub sections: Vec<Section>,
}

impl Slice {
    pub fn get_os_list(&self) -> Vec<&str> {
        let os_section = self.sections.iter()
                                      .find(|section| section.kind == Kind::Os)
                                      .unwrap();
        os_section.items.iter().map(|item| item.deref()).collect::<Vec<&str>>()
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
        version: Some(Version::parse("0.0.0").unwrap()),
        sections: sections,
    };
    let os_list = slice.get_os_list();
    assert_eq!(os_list.len(), 1);
    let os = *os_list.first().unwrap();
    assert_eq!(os, "debian-8.2");
}
