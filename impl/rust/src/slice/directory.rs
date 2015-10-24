use std::error::Error;
use std::fs::{File, metadata, read_dir};
use std::io::{ErrorKind, Read};
use std::path::Path;
use semver::Version;
use super::item::Slice;
use super::section::Section;
use helper;

/// # Examples
/// ```
/// use std::env;
/// use sb::slice::directory;
/// let current_dir = env::current_dir();
/// let mut current_dir = current_dir.unwrap().to_path_buf();
/// current_dir.push("test_slices");
/// match directory::get_latest_slice_directories(&current_dir) {
///     Ok(directories) => {
///         assert_eq!(directories, vec!["slices-1.0.0-alpha", "slices-1.0.1"]);
///     }
///     Err(error) => panic!("{}", error)
/// }
/// ```
pub fn get_latest_slice_directories(slices_directory: &Path) -> Result<Vec<String>, String> {
    match get_directories_in_directory(&slices_directory) {
        Ok(directories) => {
            if directories.is_empty() {
                return Err("There are no any slices".to_string())
            }
            let directories = get_directories_with_max_semver_major(directories);
            assert!(!directories.is_empty());
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
        slice.path = helper::relative_path_from(&slice.path, directory).unwrap();
    }
    slices
}

fn get_directories_in_directory(directory: &Path) -> Result<Vec<String>, String> {
    match read_dir(directory) {
        Ok(result) => {
            let directories = result.filter(|entry| {
                if let Ok(ref entry) = *entry {
                    metadata(entry.path()).unwrap().is_dir()
                } else {
                    false
                }
                                    })
                                    .map(|entry| {
                let path = entry.unwrap().path();
                let path = helper::relative_path_from(&path, directory);
                path.unwrap().to_str().unwrap().to_string()
                                    })
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

fn get_directories_with_max_semver_major(directories: Vec<String>) -> Vec<String> {
    if directories.is_empty() {
        return Vec::new();
    }

    let mut directories = directories.into_iter()
                                     .map(|d| {
        let (_, version) = get_slice_name_and_version_from_string(&d);
        (version, d)
                                     })
                                     .collect::<Vec<_>>();
    directories.sort_by(|a, b| b.0.cmp(&a.0));
    let major_version_max = directories[0].0.major;
    let mut directories = directories.into_iter()
                                     .filter(|d| d.0.major == major_version_max)
                                     .collect::<Vec<_>>();
    directories.sort_by(|a, b| a.0.cmp(&b.0));
    directories.into_iter().map(|d| d.1).collect::<Vec<_>>()
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

fn parse_version(string: &str) -> Version {
    if let Ok(version) = Version::parse(string) {
        version
    } else {
        Version { major: 0, minor: 0, patch: 0, pre: Vec::new(), build: Vec::new() }
    }
}

fn get_slice_name_and_version_from_string(string: &String) -> (String, Version) {
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
            (string.clone(), parse_version(""))
        }
    } else {
        if let Some(_) = string.find('.') {
            (String::new(), parse_version(&string))
        } else {
            (string.clone(), parse_version(""))
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
    let slice = Slice { name: slice_name, path: file_path.to_path_buf(), version: version,
                        sections: sections };
    Some(slice)
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_directories_with_max_semver_major() {
        let mut directories = Vec::new();
        directories.push("slices-1.0.1");
        directories.push("slices-0.2.1-beta.3");
        directories.push("slices-1.0.0-alpha");
        directories.push("slices-0.0.1-beta.3");
        let directories = directories.into_iter().map(|d| d.to_string()).collect::<Vec<_>>();
        let expected_result = vec!["slices-1.0.0-alpha", "slices-1.0.1"];
        let expected_result = expected_result.into_iter()
                                             .map(|d| d.to_string())
                                             .collect::<Vec<_>>();
        assert_eq!(super::get_directories_with_max_semver_major(directories), expected_result);
    }

    #[test]
    fn get_slice_name_and_version_from_string() {
        let string = "my-app-2.0.0-beta".to_string();
        let (slice_name, version) = super::get_slice_name_and_version_from_string(&string);
        assert_eq!(slice_name, "my-app");
        assert_eq!(version.major, 2);
        assert_eq!(version.minor, 0);
        assert_eq!(version.patch, 0);
    }
}
