extern crate semver;
pub use super::get_relative_path_from;
pub use self::item::{DependentSlice, parse_slices, Slice};
pub use self::section::Kind as SectionKind;
pub use self::section::Section;
pub mod directory;
pub mod section;
mod item;