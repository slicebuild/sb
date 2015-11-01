extern crate curl;
extern crate rustc_serialize;
extern crate semver;
extern crate zip;

pub use commands::{Command, FetchCommand, FindCommand, MakeCommand};

macro_rules! assert_not_empty {
    ($e:expr) => (assert!(!$e.is_empty(), "{} is empty", stringify!($e)));
}

use semver::Version;

pub mod commands;
pub mod options_parse;
pub mod os;
pub mod version;

mod helper;
mod slice;
mod formatters;

#[derive(Clone)]
#[derive(Copy)]
pub enum VersionMatchStrategy {
    Exact,
	ExactOrLesser,
	ExactOrGreater,
}

pub struct RequestedSlice {
    pub name: String,
	pub version: Version,
	pub version_match_strategy: VersionMatchStrategy,
}