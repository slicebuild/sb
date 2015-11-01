use std::path::Path;
use commands::command::Command;
use {RequestedSlice, VersionMatchStrategy};
use os::Os;
use slice::List;

pub struct FindCommand<'a> {
    slices: Vec<RequestedSlice>,
    os: Os,
    slice_root_directory: &'a Path,
}

impl<'a> FindCommand<'a> {
    /// # Panics
    /// if `slices` is empty
    pub fn new(slices: Vec<RequestedSlice>, os: Os, slice_root_directory: &'a Path)
               -> FindCommand<'a> {
        assert_not_empty!(slices);
        FindCommand { slices: slices, os: os, slice_root_directory: slice_root_directory }
    }

    fn print_found_requested_slices(&self, list: &List) {
        let iter = self.slices.iter();
        let slices = iter.map(|s| {
                             list.find_slice(&s.name, &s.version, s.version_match_strategy)
                         })
                         .filter(Option::is_some)
                         .map(|s| s.unwrap())
                         .collect::<Vec<_>>();
        if slices.is_empty() {
            return println!("Found requested: None");
        }
        println!("Found requested:");
        for slice in slices {
            println!("{}-{}", slice.name(), slice.version());
        }
        println!("");
    }

    fn print_similar_slices(&self, list: &List) {
        let slices = self.slices.iter()
                                .map(|s| list.find_similar_slices(&s.name))
                                .collect::<Vec<_>>();
        let mut folded_slices = Vec::new();
        for slices in &slices {
            for slice in slices {
                folded_slices.push(slice);
            }
        }
        let slices = folded_slices;
        if slices.is_empty() {
            return println!("All similar: None");
        }
        println!("All similar:");
        for slice in slices {
            println!("{}-{}", slice.name(), slice.version());
        }
        println!("");
    }

    fn print_missing_dependencies(&self, list: &List) {
        let unresolved_dependencies = list.unresolved_dependencies();
        if unresolved_dependencies.is_empty() {
            return println!("All missing: None");
        }
        println!("All missing:");
        for slice in unresolved_dependencies {
            println!("{}", slice);
        }
        println!("");
    }

    fn print_missing_requested_slices(&self, list: &List) {
        let iter = self.slices.iter();
        let slices = iter.map(|s| &s.name)
                         .filter(|s| {
                             let similar_slices = list.find_similar_slices(s);
                             similar_slices.is_empty()
                         })
                         .collect::<Vec<_>>();
        if slices.is_empty() {
            return println!("Missing requested: None");
        }
        println!("Missing requested:");
        for slice in slices {
            println!("{}", slice);
        }
    }
}

impl<'a> Command for FindCommand<'a> {
    fn run(&mut self) {
        match List::new(&self.slice_root_directory, &self.os,
                        VersionMatchStrategy::ExactOrGreater) {
            Ok(list) => {
                self.print_similar_slices(&list);
                self.print_missing_dependencies(&list);
                self.print_found_requested_slices(&list);
                self.print_missing_requested_slices(&list);
            }
            Err(error) => println!("{}", error)
        }
    }
}
