use std::cmp::Ordering;
use std::collections::HashMap;
use std::error::Error;
use std::fs::{DirEntry, File, metadata, read_dir};
use std::io::{ErrorKind, Read};
use std::path::Path;
use semver::Version;
use super::get_relative_path_from;
use super::Slice;
use super::section::Section;

pub fn get_latest_slice_directories(slices_directory: &Path) -> Result<Vec<String>, String> {
    match get_directories_in_directory(&slices_directory) {
        Ok(directories) => {
            if directories.is_empty() {
                return Err("There are no any slices".to_string())
            }
            let directories = group_directories_by_semver_major(directories);
            assert!(!directories.is_empty());
            let (_, mut directories) = directories.into_iter().max().unwrap();
            directories.sort_by(|a, b| a.version.cmp(&b.version));
            let directories = directories.into_iter();
            let directories = directories.map(|directory| directory.name)
                                         .collect();
            Ok(directories)
        }
        Err(error) => Err(error)
    }
}

pub fn get_latest_slices_from_slice_root_directory(slice_root_directory: &Path)
                                                   -> Result<Vec<Slice>, String> {
    match get_latest_slice_directories(&slice_root_directory) {
        Ok(slice_directories) => {
            let mut slices: Vec<Slice> = Vec::new();
            for directory in slice_directories {
                let mut path = slice_root_directory.to_path_buf();
                path.push(&directory);
                let slices_from_directory = get_slices_from_directory(&path);
                for slice in slices_from_directory {
                    /*
                    {
                        let added_before_slice = slices.iter().find(|other| {
                            other.name == slice.name
                        });
                        if let Some(added_before_slice) = added_before_slice {
                            panic!("{} {}", slice.path.display(), added_before_slice.path.display());
                        }
                    }
                    */
                    /*
                    if let Some(added_before_slice_position) = added_before_slice_position {
                        slices.remove(added_before_slice_position);
                    }
                    */
                    slices.push(slice);
                }
            }
            Ok(slices)
        }
        Err(error) => Err(error),
    }
}

pub fn get_slices_from_directory(directory: &Path) -> Vec<Slice> {
    let mut slices: Vec<Slice> = Vec::new();
    add_slices_from_directory(&mut slices, directory);
    for slice in &mut slices {
        slice.path = get_relative_path_from(&slice.path, directory).unwrap();
    }
    slices
}

struct SliceDirectory {
    name: String,
    version: Version,
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

fn get_directories_in_directory(directory: &Path)
    -> Result<Vec<DirEntry>, String> {
    match read_dir(directory) {
        Ok(result) => {
            let directories: Vec<DirEntry> = result.filter(|entry| {
                if let Ok(ref entry) = *entry {
                    metadata(entry.path()).unwrap().is_dir()
                } else {
                    false
                }
            })
            .map(|entry| entry.unwrap())
            .collect();
            Ok(directories)
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                let directory = directory.to_str().unwrap();
                let error = format!("Directory \"{}\" is not exist", directory);
                Err(error)
            }
            _ => Err(error.description().to_string())
        }
    }
}

fn group_directories_by_semver_major(directories: Vec<DirEntry>)
    -> HashMap<u64, Vec<SliceDirectory>> {
    let mut slice_directories: HashMap<u64, Vec<SliceDirectory>> = HashMap::new();
    for directory in directories {
        let directory = directory.path();
        let directory_name = directory.file_name().unwrap().to_str().unwrap().to_string();
        let (_, slice_version) = get_slice_name_and_version_from_string(&directory_name);
        let slice_version = slice_version.unwrap();
        let major = slice_version.major;
        let slice_directory = SliceDirectory {
            name: directory_name,
            version: slice_version,
        };
        if slice_directories.contains_key(&major) {
            let mut slice_directories_for_version = slice_directories.get_mut(&major).unwrap();
            slice_directories_for_version.push(slice_directory);
        } else {
            let mut slice_directories_for_version = Vec::new();
            slice_directories_for_version.push(slice_directory);
            slice_directories.insert(major, slice_directories_for_version);
        }
    }
    slice_directories
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
        if file_extension == "md" || file_extension == "txt" {
            return None;
        }
    }
    get_slice_from_checked_file_path(&file_path)
}

fn get_slice_from_checked_file_path(file_path: &Path) -> Option<Slice> {
    let mut file = File::open(file_path).unwrap();
    let mut content = String::new();
    let _ = file.read_to_string(&mut content).unwrap();
    let mut lines: Vec<String> = content.split('\n')
                                        .map(|n: &str| n.to_string().trim().to_string())
                                        .collect();
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
    let file_name = &file_path.file_name().unwrap().to_str().unwrap().to_string();
    let (slice_name, version) = get_slice_name_and_version_from_string(file_name);
    if let Some(version) = version {
        Some(Slice {
            name: slice_name,
            path: file_path.to_path_buf(),
            version: version,
            sections: sections,
        })
    } else {
        Some(Slice {
            name: slice_name,
            path: file_path.to_path_buf(),
            version: Version::parse("0.0.0").unwrap(),
            sections: sections,
        })
    }
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
