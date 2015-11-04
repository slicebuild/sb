use std::borrow::Borrow;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use semver::Version;
use helper;
use slice::section::SectionList;
use os::Os;
use version;

#[derive(Debug)]
pub struct Slice {
    name: String,
	version: Version,
}

impl PartialEq for Slice {
    fn eq(&self, other: &Self) -> bool {
	    self.name == other.name && self.version == other.version
	}
}

impl PartialOrd for Slice {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
	    match self.name.cmp(&other.name) {
		    Ordering::Equal => self.version.partial_cmp(&other.version),
			ordering => Some(ordering)
		}
	}
}

impl Eq for Slice {
}

impl Ord for Slice {
    fn cmp(&self, other: &Self) -> Ordering {
	    match self.name.cmp(&other.name) {
		    Ordering::Equal => self.version.cmp(&other.version),
			ordering => ordering
		}
	}
}

#[derive(Debug)]
pub struct SliceFile {
	slice: Slice,
	path: PathBuf,
}

impl PartialEq for SliceFile {
    fn eq(&self, other: &Self) -> bool {
	    self.slice.eq(&other.slice)
	}
}

impl PartialOrd for SliceFile {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
	    self.slice.partial_cmp(&other.slice)
	}
}

impl Eq for SliceFile {
}

impl Ord for SliceFile {
    fn cmp(&self, other: &Self) -> Ordering {
	    self.slice.cmp(&self.slice)
	}
}

impl SliceFile {
    fn name(&self) -> &str {
	    &self.slice.name
	}
}

#[derive(Debug)]
pub struct LoadedSlice {
	slice_file: SliceFile,
	section_list: SectionList
}

impl LoadedSlice {
    fn name(&self) -> &str {
	    self.slice_file.name()
	}

    fn path(&self) -> &Path {
		&self.slice_file.path
	}
}

impl PartialEq for LoadedSlice {
    fn eq(&self, other: &Self) -> bool {
	    self.slice_file.eq(&other.slice_file)
	}
}

impl PartialOrd for LoadedSlice {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
	    self.slice_file.partial_cmp(&other.slice_file)
	}
}

impl Eq for LoadedSlice {
}

impl Ord for LoadedSlice {
    fn cmp(&self, other: &Self) -> Ordering {
	    self.slice_file.cmp(&other.slice_file)
	}
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Eq)]
#[derive(Ord)]
pub struct DependentSlice {
    slice: LoadedSlice,
	dependencies: Dependencies,
}

impl DependentSlice {
    pub fn has_unresolved_dependencies(&self) -> bool {
		if let Dependencies::Unresolved(_) = self.dependencies {
		    true
		} else {
		    false
		}
	}

	pub fn content(&self) -> &LoadedSlice {
	    &self.slice
	}

	pub fn resolved_dependencies(&self) -> Vec<&DependentSlice> {
	    if let Dependencies::Resolved(ref dependencies) = self.dependencies {
		    dependencies.iter().map(Borrow::borrow).collect::<Vec<_>>()
		} else {
		    panic!("Resolved dependencies is not available for dependent slice \
			        with unresolved dependencies")
		}
	}

	pub fn unresolved_dependencies(&self) -> &Vec<String> {
	    if let Dependencies::Unresolved(ref dependencies) = self.dependencies {
			dependencies
		} else {
		    panic!("Unresolved dependencies is not available for dependent slice \
			        without unresolved dependencies. Use has_unresolved_dependencies method");
		}
	}
}

#[derive(Clone)]
pub struct SliceList {
	slices: Vec<Rc<DependentSlice>>,
}

impl SliceList {
    pub fn from_slices(mut slices: Vec<LoadedSlice>) -> SliceList {
	    let mut dependent_slices = Vec::new();
		while let Some(slice) = slices.pop() {
			SliceList::parse_slice(slice, &mut slices, &mut dependent_slices);
		}
		SliceList { slices: dependent_slices }
	}

	pub fn find_slice(&self, slice_name: &str, version: Option<Version>) -> Option<Rc<DependentSlice>> {
	    let mut slices = self.slices.iter().filter(|slice| {
		    slice.borrow().slice_file.slice.name == slice_name
		});
		if let Some(version) = version {
		    if let Some(slice) = slices.find(|slice| slice.content().version == version) {
			    Some(slice.clone())
			} else {
			    None
			}
		} else {
			if let Some(slice) = slices.max() {
			    Some(slice.clone())
			} else {
			    None
			}
		}
	}

	pub fn filter_for_os(&self, os: &str) -> SliceList {
	    let iter = self.slices.iter();
		let slices = iter.filter(|slice| slice.content().supports_os(os))
		                 .map(Clone::clone)
						 .collect::<Vec<_>>();
		SliceList { slices: slices }
	}

    pub fn find_similar_slices(&self, slice_name: &str) -> Vec<Rc<DependentSlice>> {
	    let iter = self.slices.iter();
	    iter.filter(|s| s.content().name.contains(slice_name))
			.map(Clone::clone)
			.collect::<Vec<_>>()
	}

	pub fn unresolved_dependencies(&self) -> Vec<String> {
	    let mut dependencies = Vec::new();
		for slice in &self.slices {
		    if slice.has_unresolved_dependencies() {
			    let slice_dependencies = slice.unresolved_dependencies();
				for dependency in slice_dependencies {
				    dependencies.push(dependency.clone());
				}
			}
		}
	    dependencies
	}

	fn parse_slice(slice: LoadedSlice, slices: &mut Vec<LoadedSlice>, dependent_slices: &mut Vec<Rc<DependentSlice>>) -> Rc<DependentSlice> {
		let mut resolved_dependencies = Vec::new();
		let mut missing_dependencies = Vec::new();
		for dependency_name in slice.section_list.dependencies() {
			if let Some(position) = slices.iter().position(|s| s.name() == *dependency_name) {
				let dependency = slices.remove(position);
				let dependency = SliceList::parse_slice(dependency, slices, dependent_slices);
				resolved_dependencies.push(dependency);
			} else {
				let dependency = dependent_slices.iter().find(|slice| {
					slice.content().name == dependency_name
				});
				if let Some(dependency) = dependency {
					resolved_dependencies.push(dependency.clone());
				} else {
					missing_dependencies.push(dependency_name.to_string());
				}
			}
		}
		let dependencies = if missing_dependencies.is_empty() {
			Dependencies::Resolved(resolved_dependencies)
		} else {
			Dependencies::Unresolved(missing_dependencies)
		};
		let slice = DependentSlice { slice: slice, dependencies: dependencies };
		let slice = Rc::new(slice);
		dependent_slices.push(slice.clone());
		slice
	}
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Eq)]
#[derive(Ord)]
enum Dependencies {
    Resolved(Vec<Rc<DependentSlice>>),
	Unresolved(Vec<String>),
}

#[derive(Debug)]
pub struct SliceDirectory {
    os_list: Vec<Os>,
	name: String,
	path: PathBuf,
	version: Version,
}

impl SliceDirectory {
    fn new(path: &Path) -> SliceDirectory {
		match SliceDirectory::get_oses(&path) {
			Ok(os_list) => {
				let dir_path = path.to_path_buf();
				let dir_name = path.file_name().unwrap().to_str().unwrap().to_string();
				let (name, version) = version::extract_name_and_version(&dir_name);
				SliceDirectory { os_list: os_list, name: name, path: dir_path,
				                 slices: None, version: version }
			}
			Err(error) => panic!("{}", error)
		}
	}

	/// # Panics
	/// If directory is not loaded
    fn supports_slice(&self, slice: &Slice, need_exact_version: bool) -> bool {
		assert!(self.is_loaded());
		self.slices.as_ref().unwrap().iter().any(|another_slice| {
		    if another_slice.slice.name == slice.name {
			    if need_exact_version {
				    another_slice.slice.version == slice.version
				} else {
				    another_slice.slice.version >= slice.version
				}
			} else {
			    false
			}
		})
	}

	fn supports_os(&self, os: &Os) -> bool {
		self.os_list.iter().any(|os_from_list| {
		    os_from_list.name == os.name && os_from_list.version >= os.version
		})
	}

	fn load(self) -> LoadedSliceDirectory {
	    LoadedSliceDirectory { slices: SliceDirectory::load_slices(&self.path) }
	}

	fn load_slices(path: &Path) -> Vec<LoadedSlice> {
	    let mut slices = Vec::new();
		let entries = fs::read_dir(path).unwrap();
		for entry in entries {
			let entry = entry.unwrap();
			let metadata = entry.metadata().unwrap();
			if metadata.is_dir() {
				let mut nested_dir_slices = SliceDirectory::load_slices(&entry.path());
				slices.append(nested_dir_slices);
			} else {
			    let file_name = entry.file_name();
				let (name, version) = version::extract_name_and_version(file_name.to_str().unwrap());
				let slice = Slice { name: name, version: version };
				let slice = SliceFile { slice: slice, path: path.to_path_buf() };
				slices.push(slice);
			}
		}
		slices
	}
}

impl PartialEq for SliceDirectory {
    fn eq(&self, other: &Self) -> bool {
		self.os_list.eq(&other.os_list) && self.name.eq(&other.name) && self.path.eq(&other.path)
		&& self.version.eq(&other.version)
	}
}

impl PartialOrd for SliceDirectory {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
	    match self.name.partial_cmp(&other.name).unwrap() {
			Ordering::Equal => {}
			ordering => return Some(ordering),
		}
		self.version.partial_cmp(&other.version)
	}
}

impl Eq for SliceDirectory {
}

impl Ord for SliceDirectory {
    fn cmp(&self, other: &Self) -> Ordering {
	    self.partial_cmp(other).unwrap()
	}
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Eq)]
pub struct SliceDirectoryList {
    directories: Vec<SliceDirectory>
}

impl SliceDirectoryList {
    /// # Panics
	/// * If slice directories root does not exist
	/// * If slice directories root does not contain any slice directories
    pub fn new<P:AsRef<Path>>(slice_directories_root: P) -> SliceDirectoryList {
		helper::assert_slice_root_exists(slice_directories_root.as_ref());
		let entries = fs::read_dir(slice_directories_root).unwrap();
		let directories = entries.map(|e| e.unwrap())
								 .filter(|e| e.metadata().unwrap().is_dir())
		                         .map(|d| SliceDirectory::new(&d.path()))
								 .collect::<Vec<_>>();
		SliceDirectoryList { directories: directories }
	}

    pub fn load(self) -> LoadedSliceDirectoryList {
		let directories = self.directories.into_iter()
		                                  .map(SliceDirectory::load)
						                  .collect::<Vec<_>>();
		LoadedSliceDirectoryList { directories: directories }
	}

	pub fn filter_for_os(self, os: &Os) -> SliceDirectoryList {
		let iter = self.directories.into_iter();
		let directories = iter.filter(|d| d.supports_os(os))
							  .collect::<Vec<_>>();
		SliceDirectoryList { directories: directories }
	}
}

pub struct LoadedSliceDirectory {
    slices: Vec<LoadedSlice>
}

pub struct LoadedSliceDirectoryList {
    directories: Vec<LoadedSliceDirectory>
}

impl LoadedSliceDirectoryList {
	pub fn to_slice_list(self) -> SliceList {
		let mut slices = Vec::new();
		for directory in self.directories {
		    let dir_slices = directory.slices;
			for slice in dir_slices {
			    slices.push(slice);
			}
		}
		SliceList::from_slices(slices)
	}
}

#[cfg(test)]
mod tests_for_dependent_slice {
    use std::path::PathBuf;
	use std::rc::Rc;
	use super::{Dependencies, DependentSlice, LoadedSlice, Slice, SliceFile, SliceList};
	use version;
	use slice::section::{Kind, Section, SectionList};

	fn create_slice(name: &str, dependencies: Vec<&str>) -> LoadedSlice {
	    let slice = Slice { name: name.to_string(), version: version::zero() };
		let slice_file = SliceFile { slice: slice, path: PathBuf::new() };
		let mut section_list = SectionList::new();
		let dependencies = dependencies.into_iter().map(str::to_string).collect::<Vec<_>>();
		section_list.add(Section { kind: Kind::Dep, items: dependencies });
		let loaded_slice_file = LoadedSlice { slice_file: slice_file,
		                                          section_list: section_list };
	}

	#[test]
	fn dependent_slice_has_unresolved_dependencies() {
	    let slice = DependentSlice { slice: create_slice("wget", Vec::new()),
		                             dependencies: Dependencies::Resolved(Vec::new()) };
		assert!(!slice.has_unresolved_dependencies());

		let slice = DependentSlice { slice: create_slice("ruby", vec!["wget"]),
		                             dependencies: Dependencies::Unresolved(vec!["wget".to_string()]) };
		assert!(slice.has_unresolved_dependencies());
	}

	#[test]
	fn dependent_slice_without_dependencies() {
		let slice = DependentSlice { slice: create_slice("wget", Vec::new()),
		                             dependencies: Dependencies::Resolved(Vec::new()) };
		assert!(!slice.has_unresolved_dependencies());
		assert!(slice.resolved_dependencies().is_empty());
	}

	#[test]
	fn dependent_slice_with_resolved_dependencies() {
		let slice = DependentSlice { slice: create_slice("wget", Vec::new()),
		                             dependencies: Dependencies::Resolved(Vec::new()) };
		let slice = Rc::new(slice);
		let slice = DependentSlice { slice: create_slice("ruby", vec!["wget"]),
		                             dependencies: Dependencies::Resolved(vec![slice]) };
		assert!(!slice.has_unresolved_dependencies());
		let dependencies = slice.resolved_dependencies();
		assert_eq!(dependencies.len(), 1);
		assert_eq!(dependencies[0].content().name(), "wget");
	}

	#[test]
	fn dependent_slice_with_unresolved_dependencies() {
		let slice = DependentSlice { slice: create_slice("ruby", vec!["wget"]),
		                             dependencies: Dependencies::Unresolved(vec!["wget".to_string()]) };
		assert!(slice.has_unresolved_dependencies());
		let dependencies = slice.unresolved_dependencies();
		assert_eq!(dependencies.len(), 1);
		assert_eq!(dependencies[0], "wget");
	}

	#[test]
	fn slice_list_from_slices() {
	    let slices = vec![create_slice("wget", Vec::new()), create_slice("ruby", vec!["wget"])];
		let slice_list = SliceList::from_slices(slices);
		assert!(slice_list.unresolved_dependencies().is_empty());

		let slice = slice_list.find_slice("wget", None).unwrap();
		assert!(!slice.has_unresolved_dependencies());
		assert!(slice.resolved_dependencies().is_empty());

		let slice = slice_list.find_slice("ruby", None).unwrap();
		assert!(!slice.has_unresolved_dependencies());
		let dependencies = slice.resolved_dependencies();
		assert_eq!(dependencies.len(), 1);
		assert_eq!(dependencies[0].content().name(), "wget");
	}

	#[test]
	fn slice_list_with_unresolved_dependencies() {
	    let slices = vec![create_slice("wget", Vec::new()),
		                  create_slice("ruby", vec!["curl"]),
						  create_slice("apache", vec!["ruby", "gooo"])];
		let slice_list = SliceList::from_slices(slices);
		let dependencies = slice_list.unresolved_dependencies();
		assert_eq!(dependencies, vec!["curl", "gooo"]);
	}
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
	use std::env;
	use std::path::PathBuf;
    use super::{Slice, SliceDirectory, SliceDirectoryList};
	use os::Os;
	use version;

	fn create_slice_directory_list() -> SliceDirectoryList {
	    let mut path = env::current_dir().unwrap().to_path_buf();
		path.push("test_slices");
	    SliceDirectoryList::new(&path)
	}

	#[test]
	fn get_oses_from_correct_slice_directory() {
		let mut path = env::current_dir().unwrap().to_path_buf();
		path.push("test_slices");
	    path.push("slices-du-0.0.2");
		let os_list = vec![Os { name: "debian".to_string(), version: version::parse("8.2") },
		                   Os { name: "ubuntu".to_string(), version: version::parse("14.3.2") }];
		assert_eq!(SliceDirectory::get_oses(&path).unwrap(), os_list);
	}

	#[test]
	fn get_oses_from_incorrect_slice_directory() {
		let mut path = env::current_dir().unwrap().to_path_buf();
		path.push("test_slices");
	    path.push("slices-du-1.0.2");
		let mut os_dir_path = path.to_path_buf();
		os_dir_path.push("_");
		assert_eq!(SliceDirectory::get_oses(&path).unwrap_err(),
		           SliceDirectory::get_error_for_nonexistent_os_dir(&os_dir_path));
	}

    #[test]
	fn slice_directory_list() {
	    let list = create_slice_directory_list();
		let os_list = vec![Os { name: "debian".to_string(), version: version::parse("8.2") },
		                   Os { name: "ubuntu".to_string(), version: version::parse("14.3.2") }];
		let name = "slices-du".to_string();
		let mut path = env::current_dir().unwrap().to_path_buf();
		path.push("test_slices");
		path.push("slices-du-0.0.2");
		let version = version::parse("0.0.2");
		let expected_directory = SliceDirectory { os_list: os_list, name: name, path: path, version: version };
		assert_eq!(list.directories, vec![expected_directory]);
	}

	#[test]
	fn load_list() {
	    let list = create_slice_directory_list();
		let list = list.load();
	}

	#[test]
	fn filter_slice_directory_list_for_valid_os() {
		let list = create_slice_directory_list();
		let os = Os { name: "debian".to_string(), version: version::zero() };
		let list = list.filter_for_os(&os);
		assert_eq!(list.directories.len(), 1);
	}

	#[test]
	fn filter_slice_directory_list_for_invalid_os() {
		let list = create_slice_directory_list();
		let os = Os { name: "bebian".to_string(), version: version::zero() };
		let list = list.filter_for_os(&os);
		assert!(list.directories.is_empty());
	}
}