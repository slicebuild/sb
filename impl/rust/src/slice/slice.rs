use std::borrow::Borrow;
use std::cmp::Ordering;
use std::rc::Rc;
use semver::Version;

#[derive(Debug)]
pub struct Slice {
    name: String,
	version: Version,
	ancestors: Vec<String>,
	preparations: Vec<String>,
	resolved_dependencies: Vec<Rc<Slice>>,
	unresolved_dependencies: Vec<String>,
}

impl Slice {
	/// # Panics
	/// * If `name` is empty
    pub fn new(name: String, version: Version, ancestors: Vec<String>, preparations: Vec<String>,
	           resolved_dependencies: Vec<Rc<Slice>>, unresolved_dependencies: Vec<String>)
			   -> Slice {
		assert_not_empty!(name);
		Slice { name: name, version: version, ancestors: ancestors, preparations: preparations,
		        resolved_dependencies: resolved_dependencies,
				unresolved_dependencies: unresolved_dependencies }
	}

	pub fn name(&self) -> &String {
	    &self.name
	}

	pub fn version(&self) -> &Version {
	    &self.version
	}

	pub fn ancestors(&self) -> &Vec<String> {
	    &self.ancestors
	}

	pub fn resolved_dependencies(&self) -> Vec<&Slice> {
		self.resolved_dependencies.iter().map(Borrow::borrow).collect()
	}

	pub fn unresolved_dependencies(&self) -> &Vec<String> {
		&self.unresolved_dependencies
	}

	pub fn unresolved_dependencies_include_nested(&self) -> Vec<&String> {
		let iter = self.unresolved_dependencies.iter();
	    let mut unresolved_dependencies = iter.map(Borrow::borrow).collect::<Vec<_>>();
		for resolved_dependency in &self.resolved_dependencies {
			for unresolved_dependency in resolved_dependency.unresolved_dependencies() {
				unresolved_dependencies.push(unresolved_dependency);
			}
		}
	    unresolved_dependencies
	}

	pub fn preparations(&self) -> &Vec<String> {
	    &self.preparations
	}
}

impl PartialEq for Slice {
    fn eq(&self, other: &Self) -> bool {
	    self.name == other.name && self.version == other.version
	}
}

impl PartialOrd for Slice {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
	    if self.name == other.name {
		    self.version.partial_cmp(&other.version)
		} else {
		    self.name.partial_cmp(&other.name)
		}
	}
}

impl Eq for Slice {
}

impl Ord for Slice {
    fn cmp(&self, other: &Self) -> Ordering {
	    self.partial_cmp(&other).unwrap()
	}
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
	use slice::Slice;
	use version;

	fn create_slice(name: &str, resolved_dependencies: Vec<Rc<Slice>>,
	                unresolved_dependencies: Vec<&str>) -> Slice {
		let unresolved_dependencies = unresolved_dependencies.into_iter().map(str::to_string)
																	     .collect();
		Slice::new(String::from(name), version::zero(), Vec::new(),
		           vec![String::from("do_nothing")], resolved_dependencies,
			       unresolved_dependencies)
	}

    #[test]
	fn unresolved_dependencies() {
	    let nested_slice = Rc::new(create_slice("slice2", Vec::new(), vec!["slice3"]));
	    let slice = create_slice("slice1", vec![nested_slice], vec!["slice4"]);
		assert_eq!(*slice.unresolved_dependencies(), vec![String::from("slice4")]);
	}

	#[test]
	fn unresolved_dependencies_include_nested() {
	    let nested_slice = Rc::new(create_slice("slice2", Vec::new(), vec!["slice3"]));
	    let slice = create_slice("slice1", vec![nested_slice], vec!["slice4"]);
		assert_eq!(slice.unresolved_dependencies_include_nested(), vec![&String::from("slice4"), &String::from("slice3")]);
	}
}