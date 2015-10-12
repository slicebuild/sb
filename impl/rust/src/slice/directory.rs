use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::{DirEntry, File, metadata, read_dir};
use std::io::Read;
use std::path::{Path};
use semver::{Version};
use super::section::Kind;
use super::section::Section;
use super::item::Slice;
#[cfg(test)]
use super::super::for_testing::get_slice_root_directory;

pub fn get_latest_slice_directories(slices_directory: &Path) -> Result<Vec<String>, String> {
    let directories: Vec<DirEntry> = read_dir(&slices_directory).unwrap().filter(|entry| {
        match *entry {
            Ok(ref entry) => {
                let entry_path = entry.path();
                let metadata = metadata(entry_path).unwrap();
                metadata.is_dir()
            },
            Err(_) => false
        }
    }).map(|entry| entry.unwrap()).collect();
    if directories.is_empty() {
        Err("There are no any slices".to_string())
    } else {
        let mut slice_directories: HashMap<u64, Vec<SliceDirectory>> = HashMap::new();
        for directory in directories {
            let directory = directory.path();
            let directory_name = directory.file_name().unwrap().to_str().unwrap().to_string();
            let (_, slice_version) = get_slice_name_and_version_from_string(&directory_name);
            let slice_version = slice_version.unwrap();
            let major = slice_version.major;
            let slice_directory = SliceDirectory { name: directory_name, version: slice_version };
            if slice_directories.contains_key(&major) {
                let mut slice_directories_for_version = slice_directories.get_mut(&major).unwrap();
                slice_directories_for_version.push(slice_directory);
            } else {
                let mut slice_directories_for_version = Vec::new();
                slice_directories_for_version.push(slice_directory);
                slice_directories.insert(major, slice_directories_for_version);
            }
        }
        assert!(!slice_directories.is_empty());
        let (_, mut slice_directories) = slice_directories.into_iter().max().unwrap();
        slice_directories.sort_by(|a, b| a.version.cmp(&b.version));
        let slice_directories = slice_directories.into_iter().map(|directory| directory.name).collect();
        Ok(slice_directories)
    }
}

pub fn get_latest_slices_from_slice_root_directory(slice_root_directory: &Path) -> Result<Vec<Slice>, String> {
    match get_latest_slice_directories(&slice_root_directory) {
        Ok(slice_directories) => {
            let mut slices: Vec<Slice> = Vec::new();
            for directory in slice_directories {
                let mut path = slice_root_directory.to_path_buf();
                path.push(&directory);
                let slices_from_directory = get_slices_from_directory(&path);
                for slice in slices_from_directory {
                    let added_before_slice_position = slices.iter().position(|other| other.name == slice.name);
                    if let Some(added_before_slice_position) = added_before_slice_position {
                        slices.remove(added_before_slice_position);
                    }
                    slices.push(slice);
                }
            }
            Ok(slices)
        },
        Err(error) => Err(error)
    }
}

pub fn get_slices_from_directory(directory: &Path) -> Vec<Slice> {
    let mut slices: Vec<Slice> = Vec::new();
    add_slices_from_directory(&mut slices, directory);
    slices
}

struct SliceDirectory {
    name: String,
    version: Version
}

impl PartialEq<SliceDirectory> for SliceDirectory {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.version == other.version
    }
}

impl Eq for SliceDirectory {
}

impl PartialOrd<SliceDirectory> for SliceDirectory {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.version.partial_cmp(&other.version)
    }
}

impl Ord for SliceDirectory {
    fn cmp(&self, other: &Self) -> Ordering {
        self.version.cmp(&other.version)
    }
}

fn add_slices_from_directory(slices: &mut Vec<Slice>, directory: &Path) {
    assert!(metadata(&directory).unwrap().is_dir());
    for entry in read_dir(&directory).unwrap() {
        let entry_path = entry.unwrap().path();
        if metadata(&entry_path).unwrap().is_dir() {
            add_slices_from_directory(slices, &entry_path);
        } else {
            if let Some(slice) = get_slice_from_file_path(&entry_path) {
                slices.push(slice);
            }
        }
    }
}

fn parse_version(string: &String) -> Option<Version> {
    if let Ok(version) = Version::parse(&string) {
        Some(version)
    } else {
        None
    }
}

fn get_slice_name_and_version_from_string(string: &String) -> (String, Option<Version>) {
    if let Some(dash_position) = string.find('-') {
        if let Some(dot_position) = string.find('.') {
            if dash_position < dot_position {
                let slice = string[0..dot_position].to_string();
                let last_dash_position = slice.rfind('-').unwrap();
                let slice_name = string[0..last_dash_position].to_string();
                let version = string[last_dash_position + 1..string.len()].to_string();
                (slice_name, parse_version(&version))
            } else {
                (String::new(), parse_version(&string))
            }
        } else {
            (string.clone(), None)
        }
    } else {
        if let Some(_) = string.find('.') {
            (String::new(), parse_version(&string))
        } else {
            (string.clone(), None)
        }
    }
}

fn get_slice_from_file_path(file_path: &Path) -> Option<Slice> {
    if let Some(file_extension) = file_path.extension() {
        let file_extension = file_extension.to_str().unwrap();
        match file_extension {
            "md" | "txt" => None,
            _ => get_slice_from_checked_file_path(&file_path)
        }
    } else {
        get_slice_from_checked_file_path(&file_path)
    }
}

fn get_slice_from_checked_file_path(file_path: &Path) -> Option<Slice> {
    let mut file = File::open(file_path).unwrap();
    let mut content = String::new();
    let _ = file.read_to_string(&mut content).unwrap();
    let mut lines: Vec<String> = content.split('\n').map(|n: &str| n.to_string().trim().to_string()).collect();
    let mut sections: Vec<Section> = Vec::new();
    while !lines.is_empty() {
        let (section, remaining_lines) = Section::load_from_lines(lines);
        if let Some(section) = section {
            sections.push(section);
            lines = remaining_lines;
        } else {
            return None
        }
    }
    let file_stem = &file_path.file_stem().unwrap().to_str().unwrap().to_string();
    let (slice_name, version) = get_slice_name_and_version_from_string(file_stem);
    let slice = Slice { name: slice_name, version: version, sections: sections };
    Some(slice)
}

#[test]
fn test_get_slice_name_and_version_from_string() {
    let string = "my-app-2.0.0-beta".to_string();
    let (slice_name, version) = get_slice_name_and_version_from_string(&string);
    assert_eq!(slice_name, "my-app");
    let version = version.unwrap();
    assert_eq!(version.major, 2);
    assert_eq!(version.minor, 0);
    assert_eq!(version.patch, 0);
}

#[test]
fn test_get_slice_from_file_path_for_correct_file() {
    let mut path = get_slice_root_directory();
    path.push("slices-0.0.1-beta.2/du/w/wget/wget");
    let slice = get_slice_from_file_path(&path);
    if let Some(slice) = slice {
        assert_eq!(slice.name, "wget");
        assert_eq!(slice.version, None);
        assert_eq!(slice.sections.len(), 3);
        let os_list = slice.get_os_list();
        assert_eq!(os_list.len(), 2);
        assert_eq!(os_list.first().unwrap(), "debian");
        assert_eq!(os_list.last().unwrap(), "ubuntu");
        for section in &slice.sections {
            match section.kind {
                Kind::Dep => {
                assert_eq!(section.items.len(), 1);
                assert_eq!(section.items.first().unwrap(), "upgrade");
                },
                Kind::Run => {
                    assert_eq!(section.items.len(), 1);
                    assert_eq!(section.items.first().unwrap(), "apt-get install wget -y");
                },
                Kind::Os => {
                },
                _ => panic!()
            }
        }
    } else {
        panic!();
    }
}

#[test]
fn test_get_slice_from_file_path_for_incorrect_file() {
    let mut path = get_slice_root_directory();
    path.push("slices-0.0.1-beta.2/du/README.md");
    let slice = get_slice_from_file_path(&path);
    if let Some(_) = slice {
        panic!();
    }
}

#[test]
fn test_get_slices_from_directory() {
    let mut path = get_slice_root_directory();
    path.push("slices-0.0.1-beta.2");
    let slices = get_slices_from_directory(&path);
    assert_eq!(slices.len(), 6);
}

//#[test]
//fn test_choose_latest_master_slice_directory_in_directory() {
//    let path = Path::new("/home/owl/sb/impl/rust/test_slices");
//    let latest_slice_directory = choose_latest_slice_directory_in_directory(&path, false).unwrap();
//    assert_eq!(latest_slice_directory, "/home/owl/sb/impl/rust/test_slices/slices-0.0.2-beta.2");
//}

//#[test]
//fn test_choose_latest_custom_slice_directory_in_directory() {
//    let path = Path::new("/home/owl/sb/impl/rust/test_slices");
//    let latest_slice_directory = choose_latest_slice_directory_in_directory(&path, true).unwrap();
//    assert_eq!(latest_slice_directory, "/home/owl/sb/impl/rust/test_slices/slices-custom-0.0.2-beta.2");
//}

#[test]
fn test_get_latest_slice_directories() {
    let path = get_slice_root_directory();
    match get_latest_slice_directories(&path) {
        Ok(directories) => assert_eq!(directories, vec!("slices-0.0.1-beta.2")),
        Err(error) => panic!(error)
    }
}