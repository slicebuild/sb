pub use helper::check_slice_root_exists;
extern crate curl;
extern crate rustc_serialize;
extern crate semver;
extern crate zip;
pub mod commands;
mod helper;
pub mod options_parse;
pub mod slice;
