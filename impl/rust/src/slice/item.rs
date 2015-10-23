use super::{Section, SectionKind};
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::ops::Deref;
use std::path::PathBuf;
use std::rc::Rc;
use semver::Version;

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
    /// use semver::Version;
    /// use sb::{Section, SectionKind, Slice};
    /// use std::path::PathBuf;
    /// fn main() {
    ///     let section = Section { kind: SectionKind::Os, items: vec!["debian".to_string()] };
    ///     let sections = vec![section];
    ///     let slice = Slice { name: "".to_string(), path: PathBuf::new(),
    ///                         version: Version::parse("0.0.0").unwrap(),
    ///                         sections: sections };
    ///     assert_eq!(slice.get_section_items(SectionKind::Os), Some(vec!["debian"]));
    ///     assert_eq!(slice.get_section_items(SectionKind::Dep), None);
    /// }
    /// ```
    pub fn get_section_items(&self, section_kind: SectionKind) -> Option<Vec<&str>> {
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

    pub fn get_os_list(&self) -> Vec<&str> {
        if let Some(os_list) = self.get_section_items(SectionKind::Os) {
            os_list
        } else {
            panic!("Slice has no os section. Slice path = {}", self.path.display());
        }
    }

    pub fn get_run_list(&self) -> Vec<&str> {
        if let Some(run_list) = self.get_section_items(SectionKind::Run) {
            run_list
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
    let os_list = slice.get_section_items(SectionKind::Os).unwrap();
    assert_eq!(os_list.len(), 1);
    let os = *os_list.first().unwrap();
    assert_eq!(os, "debian-8.2");
}

#[derive(Debug)]
pub struct DependentSlice {
    pub slice: Slice,
    pub resolved_dependencies: Vec<Rc<RefCell<DependentSlice>>>,
    pub missing_dependencies: Vec<String>
}

impl DependentSlice {
    pub fn unresolved_dependencies(&self) -> Vec<String> {
        let mut unresolved_dependencies = self.missing_dependencies.clone();
        for slice in &self.resolved_dependencies {
            let slice = slice.clone();
            let slice = slice.borrow();
            let slice_unresolved_dependencies = slice.unresolved_dependencies();
            for unresolved_dependency in slice_unresolved_dependencies {
                unresolved_dependencies.push(unresolved_dependency);
            }
        }
        unresolved_dependencies
    }
}

impl PartialOrd for DependentSlice {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.slice.partial_cmp(&other.slice)
    }
}

impl PartialEq for DependentSlice {
    fn eq(&self, other: &Self) -> bool {
        self.slice.eq(&other.slice)
    }
}

impl Ord for DependentSlice {
    fn cmp(&self, other: &Self) -> Ordering {
        self.slice.cmp(&other.slice)
    }
}

impl Eq for DependentSlice {
}

/// # Examples
/// ```
/// extern crate sb;
/// extern crate semver;
/// use semver::Version;
/// use sb::{DependentSlice, parse_slices, Section, SectionKind, Slice};
/// use std::cell::RefCell;
/// use std::path::PathBuf;
/// use std::rc::Rc;
///
/// fn create_slice(slice_name: &str, dependencies: Vec<&str>) -> Slice {
///     let iter = dependencies.into_iter();
///     let dependencies: Vec<String> = iter.map(str::to_string).collect();
///     let sections = vec![Section { kind: SectionKind::Dep, items: dependencies }];
///     Slice { name: slice_name.to_string(), path: PathBuf::new(),
///             version: Version::parse("0.0.0").unwrap(), sections: sections }
/// }
///
/// fn assert_slice_has_expected_unresolved_dependencies(slice: &Rc<RefCell<DependentSlice>>,
///                                                      unresolved_dependencies: Vec<String>) {
///     let slice = slice.clone();
///     let slice = slice.borrow();
///     assert_eq!(slice.unresolved_dependencies, unresolved_dependencies);
/// }
///
/// fn assert_slice_has_expected_resolved_dependencies(slice: &Rc<RefCell<DependentSlice>>,
///                                                    resolved_dependencies: Vec<&str>) {
///     let slice = slice.clone();
///     let slice = slice.borrow();
///     assert_eq!(slice.resolved_dependencies.len(), resolved_dependencies.len());
///     let zip_iter = slice.resolved_dependencies.iter().zip(resolved_dependencies);
///     for (returned_dependency, expected_dependency) in zip_iter {
///         let returned_dependency = returned_dependency.clone();
///         let returned_dependency = returned_dependency.borrow();
//         assert_eq!(returned_dependency.slice.name, expected_dependency);
///     }
/// }
///
/// fn assert_slice_has_expected_name(slice: &Rc<RefCell<DependentSlice>>, name: &str) {
///     let slice = slice.clone();
///     let slice = slice.borrow();
///     assert_eq!(slice.slice.name, name);
/// }
///
/// fn main() {
///     let mut slices = Vec::<Slice>::new();
///     slices.push(create_slice("jekyll", vec!["ruby"]));
///     slices.push(create_slice("ruby", vec!["wget"]));
///     slices.push(create_slice("wget", Vec::new()));
///     println!("160");
///     let slices = parse_slices(slices);
///     println!("161");
///     assert_slice_has_expected_name(&slices[0], "jekyll");
///     assert_slice_has_expected_unresolved_dependencies(&slices[0], Vec::new());
///     assert_slice_has_expected_resolved_dependencies(&slices[0], vec!["ruby"]);
///     assert_slice_has_expected_name(&slices[1], "ruby");
///     assert_slice_has_expected_unresolved_dependencies(&slices[1], Vec::new());
///     assert_slice_has_expected_resolved_dependencies(&slices[1], vec!["wget"]);
///     assert_slice_has_expected_name(&slices[2], "wget");
///     assert_slice_has_expected_unresolved_dependencies(&slices[2], Vec::new());
///     assert_slice_has_expected_resolved_dependencies(&slices[2], Vec::new());
/// }
/// ```

/// ```
/// extern crate sb;
/// extern crate semver;
/// use semver::Version;
/// use sb::{DependentSlice, parse_slices, Section, SectionKind, Slice};
/// use std::cell::RefCell;
/// use std::rc::Rc;
/// use std::path::PathBuf;
///
/// fn create_slice(slice_name: &str, dependencies: Vec<&str>) -> Slice {
///     let iter = dependencies.into_iter();
///     let dependencies: Vec<String> = iter.map(str::to_string).collect();
///     let sections = vec![Section { kind: SectionKind::Dep, items: dependencies }];
///     Slice { name: slice_name.to_string(), path: PathBuf::new(),
///             version: Version::parse("0.0.0").unwrap(), sections: sections }
/// }
///
/// fn assert_slice_has_expected_unresolved_dependencies(slice: &Rc<RefCell<DependentSlice>>,
///                                                      unresolved_dependencies: Vec<&str>) {
///     let slice = slice.clone();
///     let slice = slice.borrow();
///     assert_eq!(slice.unresolved_dependencies, unresolved_dependencies);
/// }
///
/// fn assert_slice_has_expected_resolved_dependencies(slice: &Rc<RefCell<DependentSlice>>,
///                                                    resolved_dependencies: Vec<&str>) {
///     let slice = slice.clone();
///     let slice = slice.borrow();
///     assert_eq!(slice.resolved_dependencies.len(), resolved_dependencies.len());
///     let zip_iter = slice.resolved_dependencies.iter().zip(resolved_dependencies);
///     for (returned_dependency, expected_dependency) in zip_iter {
///         let returned_dependency = returned_dependency.clone();
///         let returned_dependency = returned_dependency.borrow();
///         assert_eq!(returned_dependency.slice.name, expected_dependency);
///     }
/// }
///
/// fn assert_slice_has_expected_name(slice: &Rc<RefCell<DependentSlice>>, name: &str) {
///     let slice = slice.clone();
///     let slice = slice.borrow();
///     assert_eq!(slice.slice.name, name);
/// }
///
/// fn main() {
///     let mut slices = Vec::<Slice>::new();
///     slices.push(create_slice("jekyll", vec!["ruby"]));
///     slices.push(create_slice("wget", Vec::new()));
///     let slices = parse_slices(slices);
///     assert_slice_has_expected_name(&slices[0], "jekyll");
///     assert_slice_has_expected_unresolved_dependencies(&slices[0], vec!["ruby"]);
///     assert_slice_has_expected_resolved_dependencies(&slices[0], Vec::new());
///     assert_slice_has_expected_name(&slices[1], "wget");
///     assert_slice_has_expected_unresolved_dependencies(&slices[1], Vec::new());
///     assert_slice_has_expected_resolved_dependencies(&slices[1], Vec::new());
/// }
/// ```
pub fn parse_slices(slices: Vec<Slice>) -> Vec<Rc<RefCell<DependentSlice>>> {
    let slices = slices.into_iter().map(|slice| {
        let slice = DependentSlice { slice: slice, resolved_dependencies: Vec::new(),
                                     missing_dependencies: Vec::new() };
        Rc::new(RefCell::new(slice))
    });
    let slices = slices.collect::<Vec<_>>();
    for slice in &slices {
        set_slice_dependencies(slice.clone(), &slices);
    }
    slices
}

fn set_slice_dependencies(slice: Rc<RefCell<DependentSlice>>, slices: &Vec<Rc<RefCell<DependentSlice>>>) {
    let dependency_names = {
        let slice = slice.clone();
        let slice = slice.borrow();
        if let Some(dependency_names) = (*slice).slice.get_section_items(SectionKind::Dep) {
            dependency_names.iter().map(|d| d.to_string()).collect::<Vec<String>>()
        } else {
            Vec::new()
        }
    };
    for dependency_name in dependency_names {
        let mut iter = slices.iter();
        let dependency_slice = iter.find(|dependency_slice| {
            let dependency_slice = dependency_slice.clone();
            if *dependency_slice == slice {
                return false;
            }
            let dependency_slice = dependency_slice.borrow();
            dependency_slice.slice.name == dependency_name
        });
        let slice = slice.clone();
        let mut slice = (*slice).borrow_mut();
        if let Some(dependency_slice) = dependency_slice {
            (*slice).resolved_dependencies.push(dependency_slice.clone());
        } else {
            (*slice).missing_dependencies.push(dependency_name.to_string());
        }
    }
}
