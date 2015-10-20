use std::io::ErrorKind;
use std::fs::metadata;
use std::path::Path;

/// # Examples
/// ```should_panic
/// use sb::check_slice_root_exists;
/// use std::path::Path;
/// let path = Path::new("/path/that/does/not/exist");
/// check_slice_root_exists(path);
/// ```

/// ```
/// use sb::check_slice_root_exists;
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
