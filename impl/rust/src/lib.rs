extern crate curl;
extern crate rustc_serialize;
extern crate semver;
extern crate zip;

#[macro_export]
macro_rules! assert_not_empty {
    ($e:expr) => (assert!(!$e.is_empty(), "{} is empty", stringify!($e)));
}

pub mod commands;
pub mod helper;
pub mod options_parse;
pub mod slice;
mod formatters;
mod version;