use semver::Version;

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Eq)]
pub struct Os {
    pub name: String,
	pub version: Version,
}
