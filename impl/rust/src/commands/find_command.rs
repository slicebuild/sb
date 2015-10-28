use std::path::Path;
use helper;
use slice::directory;
use slice::list::SliceList;
use super::command::Command;

pub struct FindCommand<'a> {
    layers: &'a Vec<&'a str>,
    os: &'a str,
    slice_root_directory: &'a Path,
}

impl<'a> FindCommand<'a> {
    pub fn new(layers: &'a Vec<&'a str>, os: &'a str,
               slice_root_directory: &'a Path) -> FindCommand<'a> {
        assert!(!layers.is_empty());
        assert!(!os.is_empty());
        FindCommand { layers: layers, os: os,
                      slice_root_directory: slice_root_directory }
    }

    fn print_found_requested_slices_section(&self, slice_list: &SliceList) {
        let iter = self.layers.iter();
        let slices = iter.map(|layer| slice_list.find_slice(layer, None))
                         .filter(|slice| slice.is_some())
                         .collect::<Vec<_>>();
        if slices.is_empty() {
            return println!("Found requested: None");
        }
        println!("Found requested:");
        for slice in slices {
            println!("{}", slice.unwrap().content().path.display());
        }
        println!("");
    }

    fn print_similar_slices_section(&self, slice_list: &SliceList) {
        let iter = self.layers.iter();
        let slices_list = iter.map(|layer| slice_list.find_similar_slices(layer));
        let mut similar_slices = Vec::new();
        for slices in slices_list {
            for slice in slices {
                similar_slices.push(slice);
            }
        }
        if similar_slices.is_empty() {
            return println!("All similar: None");
        }
        println!("All similar:");
        for slice in similar_slices {
            println!("{}", slice.content().path.display());
        }
        println!("");
    }

    fn print_missing_dependencies_section(&self, slice_list: &SliceList) {
        let unresolved_dependencies = slice_list.unresolved_dependencies();
        if unresolved_dependencies.is_empty() {
            return println!("All missing: None");
        }
        println!("All missing:");
        for slice in unresolved_dependencies {
            println!("{}", slice);
        }
        println!("");
    }

    fn print_missing_requested_slices_section(&self, slice_list: &SliceList) {
        let iter = self.layers.iter();
        let slices = iter.filter(|layer| slice_list.find_similar_slices(layer).is_empty())
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
        helper::assert_slice_root_exists(self.slice_root_directory);
        match directory::get_latest_slices_from_slice_root_directory(&self.slice_root_directory) {
            Ok(slices) => {
                let slice_list = SliceList::from_slices(slices);
                let slice_list = slice_list.filter_for_os(self.os);
                self.print_similar_slices_section(&slice_list);
                self.print_missing_dependencies_section(&slice_list);
                self.print_found_requested_slices_section(&slice_list);
                self.print_missing_requested_slices_section(&slice_list);
            }
            Err(error) => println!("{}", error),
        }
    }
}
