use std::borrow::Borrow;
use std::rc::Rc;
use semver::Version;
use super::item::Slice;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Eq)]
#[derive(Ord)]
pub struct DependentSlice {
    slice: Slice,
	dependencies: Dependencies
}

impl DependentSlice {
    pub fn has_unresolved_dependencies(&self) -> bool {
		if let Dependencies::Unresolved(_) = self.dependencies {
		    true
		} else {
		    false
		}
	}

	pub fn content(&self) -> &Slice {
	    &self.slice
	}

	pub fn resolved_dependencies(&self) -> Vec<&DependentSlice> {
	    if let Dependencies::Resolved(ref dependencies) = self.dependencies {
		    dependencies.iter().map(|d| d.borrow()).collect::<Vec<&DependentSlice>>()
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
    pub fn from_slices(mut slices: Vec<Slice>) -> SliceList {
	    let mut dependent_slices = Vec::new();
		while let Some(slice) = slices.pop() {
			SliceList::parse_slice(slice, &mut slices, &mut dependent_slices);
		}
		SliceList { slices: dependent_slices }
	}

	pub fn find_slice(&self, slice_name: &str, version: Option<Version>) -> Option<Rc<DependentSlice>> {
	    let mut slices = self.slices.iter().filter(|slice| {
		    slice.content().name == slice_name
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
		                 .map(|slice| slice.clone())
						 .collect::<Vec<_>>();
		SliceList { slices: slices }
	}

    pub fn find_similar_slices(&self, slice_name: &str) -> Vec<Rc<DependentSlice>> {
	    let iter = self.slices.iter();
	    iter.filter(|s| s.content().name.contains(slice_name))
			.map(|s| s.clone())
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

	fn parse_slice(slice: Slice, slices: &mut Vec<Slice>, dependent_slices: &mut Vec<Rc<DependentSlice>>) -> Rc<DependentSlice> {
		let mut resolved_dependencies = Vec::new();
		let mut missing_dependencies = Vec::new();
		for dependency_name in slice.dependencies() {
			if let Some(position) = slices.iter().position(|s| s.name == dependency_name) {
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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
	use std::rc::Rc;
	use super::{Dependencies, DependentSlice, SliceList};
	use helper;
    use slice::item::Slice;
	use slice::section::{Kind, Section};

	fn create_slice(name: &str, dependencies: Vec<&str>) -> Slice {
	    let dependencies = dependencies.into_iter().map(|d| d.to_string()).collect::<Vec<_>>();
	    let section = Section { kind: Kind::Dep, items: dependencies };
	    let sections = vec![section];
	    Slice { name: name.to_string(), path: PathBuf::new(),
		        version: helper::zero_version(), sections: sections }
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
		assert_eq!(dependencies[0].content().name, "wget");
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
		assert_eq!(dependencies[0].content().name, "wget");
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