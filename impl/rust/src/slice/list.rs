use std::borrow::Borrow;
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter, Result as FormatResult};
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::{ErrorKind, Read};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use semver::Version;
use os::Os;
use slice::Slice;
use slice::section::{Kind, Section};
use version;
use VersionMatchStrategy;

#[derive(Debug)]
pub struct List {
	slices: Vec<Rc<Slice>>,
}

impl List {
    pub fn new(root: &Path, os: &Os, os_version_match_strategy: VersionMatchStrategy) -> Result<List, String> {
        let directories = try!(List::directory_paths_from_slice_root_directory(root));
        let mut slices = Vec::new();
        for dir in directories {
            let slices_from_path = List::slices_from_path_for_os(&dir, &os,
                                                                 &os_version_match_strategy);
            let mut slices_from_path = try!(slices_from_path);
            slices.append(&mut slices_from_path);
        }
        assert!(!slices.is_empty());
        Ok(List { slices: List::convert_blanks_to_slices(slices) })
	}

	/// # Panics
	/// If `name` is empty
	pub fn find_similar_slices(&self, name: &str) -> Vec<&Slice> {
	    assert!(!name.is_empty());
	    self.slices.iter().filter(|s| s.name().contains(name)).map(Borrow::borrow).collect()
	}

	/// # Panics
	/// If `name` is empty
	pub fn find_slice(&self, name: &str, version: &Version,
	                  version_match_strategy: VersionMatchStrategy) -> Option<&Slice> {
		let slices = self.slices.iter().filter(|s| s.name() == name);
		let slices = slices.filter(|s| {
		    match version_match_strategy {
			    VersionMatchStrategy::Exact => s.version() == version,
				VersionMatchStrategy::ExactOrLesser => s.version() <= version,
				VersionMatchStrategy::ExactOrGreater => s.version() >= version
			}
		});
		slices.max().map(Borrow::borrow)
	}

	pub fn unresolved_dependencies(&self) -> Vec<&String> {
		let mut unresolved_dependencies = Vec::new();
		for slice in &self.slices {
			for dep in slice.unresolved_dependencies_include_nested() {
				unresolved_dependencies.push(dep);
			}
		}
		unresolved_dependencies.dedup();
		unresolved_dependencies
	}

    fn convert_blanks_to_slices(mut blanks: Vec<SliceBlank>) -> Vec<Rc<Slice>> {
        let mut slices = Vec::new();
        while let Some(blank) = blanks.pop() {
            let slice = List::convert_blank_to_slice(blank, &mut blanks, &mut slices);
            slices.push(slice);
        }
        slices
    }

    fn convert_blank_to_slice(blank: SliceBlank, blanks: &mut Vec<SliceBlank>,
                              slices: &mut Vec<Rc<Slice>>) -> Rc<Slice> {
        if blank.dependencies.is_empty() {
            let slice = Slice::new(blank.name, blank.version, blank.ancestors,
                                   blank.preparations, Vec::new(), Vec::new());
            return Rc::new(slice)
        }
        let mut resolved_dependencies = Vec::new();
        let mut unresolved_dependencies = Vec::new();
        for dependency in &blank.dependencies {
        //
            let dep_blank = blanks.iter()
                                  .enumerate()
                                  .filter(|&(_, b)| b.name == *dependency)
                                  .map(|(i, b)| (b.version.clone(), i))
                                  .max();
            let dep_slice = slices.iter()
                                  .enumerate()
                                  .filter(|&(_, s)| s.name() == dependency)
                                  .map(|(i, s)| (s.version().clone(), i))
                                  .max();
            if let Some((dep_slice_version, dep_slice_position)) = dep_slice {
                if let Some((dep_blank_version, dep_blank_position)) = dep_blank {
                    if dep_slice_version < dep_blank_version {
                        let blank = blanks.remove(dep_blank_position);
                        let dependency = List::convert_blank_to_slice(blank, blanks, slices);
                        resolved_dependencies.push(dependency.clone());
                        slices.push(dependency);
                    } else {
                        resolved_dependencies.push(slices[dep_slice_position].clone());
                    }
                } else {
                    resolved_dependencies.push(slices[dep_slice_position].clone());
                }
            } else {
                if let Some((_, dep_blank_position)) = dep_blank {
                    let blank = blanks.remove(dep_blank_position);
                    let dependency = List::convert_blank_to_slice(blank, blanks, slices);
                    resolved_dependencies.push(dependency.clone());
                    slices.push(dependency);
                } else {
                    unresolved_dependencies.push(dependency.clone());
                }
            }
        }
        let slice = Slice::new(blank.name, blank.version, blank.ancestors, blank.preparations,
                               resolved_dependencies, unresolved_dependencies);
        Rc::new(slice)
    }

    fn slices_from_path_for_os(path: &Path, os: &Os,
                               os_version_match_strategy: &VersionMatchStrategy)
                               -> Result<Vec<SliceBlank>, String> {
	    let oses = try!(List::get_oses(&path));
        let directory_contains_required_os = oses.iter().any(|os_from_list| {
            if os_from_list.name != os.name {
                return false;
            }
            match *os_version_match_strategy {
                VersionMatchStrategy::Exact => os_from_list.version == os.version,
                VersionMatchStrategy::ExactOrLesser => os_from_list.version <= os.version,
                VersionMatchStrategy::ExactOrGreater => os_from_list.version >= os.version
            }
        });
		if !directory_contains_required_os {
			return Ok(Vec::new());
		}
		Ok(List::slices_from_path(path))
	}

	fn slices_from_path(path: &Path) -> Vec<SliceBlank> {
		let mut slices = Vec::new();
		let entries = fs::read_dir(path).unwrap();
		for entry in entries {
			let entry = entry.unwrap();
			let metadata = entry.metadata().unwrap();
			if metadata.is_dir() {
				let mut nested_dir_slices = List::slices_from_path(&entry.path());
				slices.append(&mut nested_dir_slices);
			} else {
			    let path = entry.path();
				if let Some(extension) = path.extension() {
				    match extension.to_str().unwrap() {
					    "txt" => continue,
						"md" => continue,
						_ => {}
					}
				}
				slices.push(List::slice_from_path(&path))
			}
		}
		slices
	}

	fn slice_from_path(path: &Path) -> SliceBlank {
	    let file_name = path.file_name().map(OsStr::to_str).unwrap().unwrap();
		let (name, version) = version::extract_name_and_version(&file_name);
		match File::open(path) {
		    Ok(mut file) => List::slice_from_file(name, version, path, &mut file),
			Err(error) => panic!("File at path = {} was not open because of error: {}",
				       			 path.display(), error)
		}
	}

    fn slice_from_file(name: String, version: Version, path: &Path, file: &mut File)
                       -> SliceBlank {
		let mut file_content = String::new();
		match file.read_to_string(&mut file_content) {
			Ok(_) => {
			    let lines = file_content.split('\n')
				                        .map(|n: &str| n.trim().to_string())
										.collect();
                SliceBlank::from_lines(name, version, lines)
			}
			Err(error) => panic!("File at path = {} cannot be read because of error: {}",
			                     path.display(), error)
		}
	}

    fn directory_paths_from_slice_root_directory(path: &Path) -> Result<Vec<PathBuf>, String> {
        match fs::read_dir(path) {
		    Ok(entries) => Ok(entries.map(Result::unwrap)
                                     .filter(|e| e.metadata().unwrap().is_dir())
                                     .map(|e| e.path())
                                     .collect()),
			Err(ref error) if error.kind() == ErrorKind::NotFound => {
			    let error = format!("Slice root directory is not exists. Path = {}",
				                    path.display());
				Err(error)
			}
			Err(error) => Err(format!("Unknown error = {}", error))
		}
    }

	fn get_oses(path: &Path) -> Result<Vec<Os>, String> {
	    let mut path = path.to_path_buf();
		path.push("_");
		match fs::read_dir(&path) {
		    Ok(entries) => {
			    let os_list = entries.map(|e| e.unwrap())
				                     .filter(|e| e.metadata().unwrap().is_file())
									 .map(|f| {
					let file_name = f.path().file_name().unwrap().to_str().unwrap().to_string();
					let (name, version) = version::extract_name_and_version(&file_name);
					Os { name: name, version: version }
									 })
									 .collect::<Vec<_>>();
				Ok(os_list)
			}
			Err(ref error) if error.kind() == ErrorKind::NotFound => {
				Err(get_error_for_nonexistent_os_dir(path))
			}
			Err(error) => Err(format!("{}", error))
		}
	}
}

fn get_error_for_nonexistent_os_dir<P:AsRef<Path>>(path: P) -> String {
	format!("There is no \"_\" directory at {}", path.as_ref().display())
}

#[derive(Hash)]
struct SliceBlank {
    name: String,
	version: Version,
	ancestors: Vec<String>,
	dependencies: Vec<String>,
	preparations: Vec<String>
}

impl SliceBlank {
    fn from_lines<L:Borrow<str>+Debug>(name: String, version: Version, mut lines: Vec<L>)
	                             -> SliceBlank {
		let mut slice_blank = SliceBlank { name: name, version: version, ancestors: Vec::new(),
                                           dependencies: Vec::new(), preparations: Vec::new() };
        while !lines.is_empty() {
            let (section, remaining_lines) = Section::from_lines(lines);
            lines = remaining_lines;
            if let Some(section) = section {
                match section.kind {
                    Kind::Dep => slice_blank.dependencies = section.items,
                    Kind::From => slice_blank.ancestors = section.items,
                    Kind::Run => slice_blank.preparations = section.items,
                    _ => {}
                }
            } else {
                break;
            }
        }
        slice_blank
	}
}

impl PartialEq for SliceBlank {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.version == other.version
    }
}

impl PartialOrd for SliceBlank {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.version.partial_cmp(&other.version)
    }
}

impl Eq for SliceBlank {
}

impl Ord for SliceBlank {
    fn cmp(&self, other: &Self) -> Ordering {
        self.version.cmp(&other.version)
    }
}

impl Debug for SliceBlank {
    fn fmt(&self, formatter: &mut Formatter) -> FormatResult {
        write!(formatter, "SliceBlank({})", self.name)
    }
}

#[cfg(test)]
mod tests {
	use std::env;
	use std::path::Path;
	use std::rc::Rc;
	use os::Os;
	use slice::{List, Slice};
	use VersionMatchStrategy;
	use version;

    #[test]
    fn directory_paths_from_test_slices_root_directory() {
        let mut path = env::current_dir().expect("Current directory is not set").to_path_buf();
		path.push("test_slices");
        match List::directory_paths_from_slice_root_directory(&path) {
            Ok(directory_paths) => {
                let expected_directory_paths = vec!["slices-du-0.0.2"];
                let expected_directory_paths = expected_directory_paths.into_iter().map(|p| {
                    let mut path = path.clone();
                    path.push(p);
                    path
                })
                .collect::<Vec<_>>();
                assert_eq!(directory_paths, expected_directory_paths);
            }
            Err(error) => panic!("{}", error)
        }
    }

    #[should_panic]
    #[test]
    fn directory_paths_from_nonexistent_slices_root_directory() {
        let mut path = env::current_dir().expect("Current directory is not set").to_path_buf();
		path.push("nonexistent_test_slices");
        match List::directory_paths_from_slice_root_directory(&path) {
            Ok(directory_paths) => panic!("Return: {:?}", directory_paths),
            Err(error) => panic!("{}", error)
        }
    }

    #[test]
    fn get_oses_for_test_slices() {
        let mut path = env::current_dir().expect("Current directory is not set").to_path_buf();
		path.push("test_slices");
        path.push("slices-du-0.0.2");
        let oses = List::get_oses(&path).expect("Oses were not retrieved");
        assert_eq!(oses, vec![Os { name: String::from("debian"), version: version::parse("8.2") },
                              Os { name: String::from("ubuntu"),
                                   version: version::parse("14.3.2") }]);
    }

    #[test]
	fn list_from_test_slices() {
	    let mut path = env::current_dir().expect("Current directory is not set").to_path_buf();
		path.push("test_slices");
	    let os = Os { name: String::from("debian"), version: version::zero() };
	    let list = List::new(&path, &os, VersionMatchStrategy::ExactOrGreater).unwrap();
        match list.find_slice("jekyll", &version::zero(), VersionMatchStrategy::ExactOrGreater) {
            Some(_) => {}
            None => panic!("Requested slice was not found")
        }
	}

	#[should_panic]
	#[test]
	fn list_from_nonexistent_directory() {
	    let path = Path::new("/non/existent/directory");
		let os = Os { name: String::from("debian"), version: version::zero() };
	    let _ = List::new(&path, &os, VersionMatchStrategy::ExactOrGreater).unwrap();
	}

	#[test]
	fn find_similar_slices() {
	    let create_slice = |name| {
		    Slice::new(String::from(name), version::zero(), Vec::new(),
			           vec![String::from("nothing")], Vec::new(), Vec::new())
		};
	    let slices = vec![create_slice("a"), create_slice("ab"), create_slice("bc")];
		let slices = slices.into_iter().map(|s| Rc::new(s)).collect();
	    let list = List { slices: slices };
		let similar_slices = list.find_similar_slices("a");
		let similar_slice_names = similar_slices.into_iter().map(Slice::name).collect::<Vec<_>>();
		assert_eq!(similar_slice_names, vec!["a", "ab"]);
	}

	fn prepare_list_to_test_find_slice() -> List {
		let create_slice_with_version = |name, version| {
		    let name = String::from(name);
			let preparations = vec![String::from("nothing")];
			Slice::new(name, version, Vec::new(), preparations, Vec::new(), Vec::new())
		};
		let create_slice = |name| {
		    create_slice_with_version(name, version::zero())
		};
	    let slices = vec![create_slice("a"),
		                  create_slice_with_version("a", version::parse("0.5.0")),
		                  create_slice_with_version("a", version::parse("1.0.0"))];
		let slices = slices.into_iter().map(|s| Rc::new(s)).collect();
	    List { slices: slices }
	}

	#[test]
	fn find_slice_with_exact_version() {
		let list = prepare_list_to_test_find_slice();
		let slice = list.find_slice("a", &version::parse("0.5.0"), VersionMatchStrategy::Exact);
		let slice = slice.expect("slice was not found");
		assert_eq!(slice.name(), "a");
		assert_eq!(*slice.version(), version::parse("0.5.0"));
	}

	#[test]
	fn find_slice_with_exact_or_lesser_version() {
		let list = prepare_list_to_test_find_slice();
		let slice = list.find_slice("a", &version::parse("0.7.0"), VersionMatchStrategy::ExactOrLesser);
		let slice = slice.expect("slice was not found");
		assert_eq!(slice.name(), "a");
		assert_eq!(*slice.version(), version::parse("0.5.0"));
	}

	#[test]
	fn find_slice_with_exact_or_greater_version() {
		let list = prepare_list_to_test_find_slice();
		let slice = list.find_slice("a", &version::parse("0.7.0"), VersionMatchStrategy::ExactOrGreater);
		let slice = slice.expect("slice was not found");
		assert_eq!(slice.name(), "a");
		assert_eq!(*slice.version(), version::parse("1.0.0"));
	}

	#[test]
	fn unresolved_dependencies() {
		let create_slice = |name, resolved_dependencies, unresolved_dependencies: Vec<&str>| {
		    let unresolved_dependencies = unresolved_dependencies.into_iter().map(str::to_string)
			                                                                 .collect::<Vec<_>>();
		    let slice = Slice::new(String::from(name), version::zero(), Vec::new(),
			                       vec![String::from("nothing")], resolved_dependencies,
								   unresolved_dependencies);
			Rc::new(slice)
		};
		let first_slice = create_slice("first_slice", Vec::new(), vec!["missing_dep_from_first_slice"]);
		let second_slice = create_slice("second_slice", vec![first_slice.clone()], Vec::new());
		let third_slice = create_slice("third_slice", Vec::new(), vec!["missing_dep_from_third_slice"]);
	    let list = List { slices: vec![first_slice, second_slice, third_slice] };
		let unresolved_deps = list.unresolved_dependencies();
		assert_eq!(unresolved_deps, vec![&String::from("missing_dep_from_first_slice"),
										 &String::from("missing_dep_from_third_slice")]);
	}
}