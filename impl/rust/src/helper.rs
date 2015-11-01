use std::io::ErrorKind;
use std::fs::metadata;
use std::path::Path;

pub fn assert_slice_root_exists(slice_root_directory: &Path) {
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

#[cfg(test)]
mod tests {
    use std::env::current_dir;
    use std::path::Path;

    #[test]
    fn test_assert_slice_root_exists_for_existent_directory() {
        let path = current_dir().unwrap();
        super::assert_slice_root_exists(&path);
    }

    #[should_panic]
    #[test]
    fn test_assert_slice_root_exists_for_nonexistent_directory() {
        let path = Path::new("/path/that/does/not/exist");
        super::assert_slice_root_exists(path);
    }
}