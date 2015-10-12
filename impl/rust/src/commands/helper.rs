use std::io::ErrorKind;
use std::fs::metadata;
use std::path::Path;

pub fn check_slice_root_exists(slice_root_directory: &Path) {
	if let Err(error) = metadata(slice_root_directory) {
		if error.kind() == ErrorKind::NotFound {
			println!("Slice root directory is not exists. Path = {}", slice_root_directory.to_str().unwrap());
		} else {
			println!("Unknown error = {}", error);
		}
		panic!();
	}
}
