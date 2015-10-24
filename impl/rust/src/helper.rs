use std::io::ErrorKind;
use std::fs::metadata;
use std::path::{Path, PathBuf};
use semver::Version;

/// # Examples
/// ```should_panic
/// use sb::helper::check_slice_root_exists;
/// use std::path::Path;
/// let path = Path::new("/path/that/does/not/exist");
/// check_slice_root_exists(path);
/// ```

/// ```
/// use sb::helper::check_slice_root_exists;
/// use std::env::current_dir;
/// use std::path::Path;
/// let path = current_dir().unwrap();
/// check_slice_root_exists(&path);
/// ```
pub fn check_slice_root_exists(slice_root_directory: &Path) {
    if let Err(error) = metadata(slice_root_directory) {
        if error.kind() == ErrorKind::NotFound {
            println!("Slice root directory is not exists. Path = {}",
                     slice_root_directory.to_str().unwrap());
        } else {
            println!("Unknown error = {}", error);
        }
        panic!();
    }
}

/// # Examples
/// ```
/// use sb::helper::relative_path_from;
/// use std::path::Path;
/// let path = Path::new("/base/path");
/// let base = Path::new("/base");
/// let relative_path = relative_path_from(path, base);
/// assert_eq!(relative_path, Some(Path::new("path").to_path_buf()));
/// ```

/// ```
/// use std::path::Path;
/// use sb::helper::relative_path_from;
/// let path = Path::new("/base/path");
/// let wrong_base = Path::new("/wrong_base");
/// let relative_path = relative_path_from(path, wrong_base);
/// assert_eq!(relative_path, None);
/// ```
pub fn relative_path_from(path: &Path, base: &Path) -> Option<PathBuf> {
    let path = path.to_str().unwrap();
    let base = base.to_str().unwrap();
    if !path.starts_with(base) {
        return None
    }
    let path = &path[base.len()..];
    let path = path.trim_matches(|c| c == '/' || c == '\\');
    let path = Path::new(path).to_path_buf();
    Some(path)
}

pub fn zero_version() -> Version {
    Version { major: 0, minor: 0, patch: 0, pre: Vec::new(), build: Vec::new() }
}