use std::env::current_dir;
use std::path::PathBuf;

pub fn get_slice_root_directory() -> PathBuf {
	let mut slice_root_directory = current_dir().unwrap();
	slice_root_directory.push("target");
	let build_type = if cfg!(debug_assertions) {
		"debug"
	} else {
		"release"
	};
	slice_root_directory.push(build_type);
	slice_root_directory.push(".cb");
	slice_root_directory.push("slices");
	slice_root_directory.to_path_buf()
}