use std::cell::RefCell;
use std::collections::HashSet;
use std::path::Path;
use std::rc::Rc;
use super::{check_slice_root_exists, Command, DependentSlice, parse_slices, SectionKind, Slice};
use super::super::slice::directory::get_latest_slices_from_slice_root_directory;

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

    fn is_slice_name_similar(&self, slice_name: &String) -> bool {
        self.layers.iter().any(|layer| layer == slice_name)
    }

    fn is_slice_supports_os(&self, slice: &Slice) -> bool {
        let os_list = slice.get_os_list();
        os_list.contains(&self.os)
    }

    fn print_found_requested_slices_section(&self, slices: &Vec<Rc<RefCell<DependentSlice>>>) {
        let slices = slices.iter().filter(|slice| {
            let slice = slice.clone();
            let slice = slice.borrow();
            slice.missing_dependencies.is_empty()
        });
        let slices = slices.collect::<Vec<&Rc<RefCell<DependentSlice>>>>();
        let mut found_slice_paths = Vec::new();
        for layer in self.layers {
            let slices = slices.iter().filter(|slice| {
                let slice = slice.clone();
                let slice = slice.borrow();
                slice.slice.name == *layer
            });
            let mut best_slice_path = None;
            let mut best_slice_version = None;
            for slice in slices {
                let slice = slice.clone();
                let slice = slice.borrow();
                if let Some(previous_best_slice_version) = best_slice_version {
                    if previous_best_slice_version <= slice.slice.version {
                        best_slice_version = Some(slice.slice.version.clone());
                        best_slice_path = Some(slice.slice.path.clone());
                    } else {
                        best_slice_version = Some(previous_best_slice_version);
                    }
                } else {
                    best_slice_path = Some(slice.slice.path.clone());
                    best_slice_version = Some(slice.slice.version.clone());
                }
            }
            if let Some(best_slice_path) = best_slice_path {
                found_slice_paths.push(best_slice_path);
            }
        }
        if found_slice_paths.len() == 0 {
            println!("Found requested: None");
        } else {
            println!("Found requested:");
            for path in found_slice_paths {
                println!("{}", path.display());
            }
        }
    }

    fn print_similar_slices_section(&self, slices: &Vec<Rc<RefCell<DependentSlice>>>) {
        let similar_slices = slices.iter().filter(|slice| {
            let slice = slice.clone();
            let slice = slice.borrow();
            self.is_slice_name_similar(&slice.slice.name)
            && self.is_slice_supports_os(&slice.slice)
        });
        let similar_slices = similar_slices.map(|slice| {
            let slice = slice.clone();
            let slice = slice.borrow();
            slice.slice.path.clone()
        });
        let similar_slices = similar_slices.collect::<Vec<_>>();
        if similar_slices.len() == 0 {
            return println!("All similar: None");
        }

        println!("All similar:");
        for slice in similar_slices {
            println!("{}", slice.display());
        }
    }

    fn print_missing_dependencies_section(&self, slices: &Vec<Rc<RefCell<DependentSlice>>>) {
        let mut missing_dependencies = HashSet::new();
        for slice in slices {
            let slice = slice.clone();
            let slice = slice.borrow();
            for dependency in &slice.missing_dependencies {
                missing_dependencies.insert(dependency.to_string());
            }
        }
        if missing_dependencies.len() == 0 {
            return println!("All missing: None");
        }

        println!("All missing:");
        for slice in missing_dependencies {
            println!("{}", slice);
        }
    }

    fn print_missing_requested_slices_section(&self, slices: &Vec<Rc<RefCell<DependentSlice>>>) {
        let missing_requested_slices = self.layers.iter().filter(|layer| {
            let layer: &str = layer;
            !slices.iter().any(|slice| {
                let slice = slice.clone();
                let slice = slice.borrow();
                let name: String = slice.slice.name.to_string();
                name.contains(layer)
            })
        });
        let missing_requested_slices = missing_requested_slices.collect::<Vec<_>>();
        if missing_requested_slices.len() == 0 {
            println!("Missing requested: None");
        } else {
            println!("Missing requested:");
            for slice in &missing_requested_slices {
                println!("{}", slice);
            }
        }
    }
}

impl<'a> Command for FindCommand<'a> {
    fn run(&mut self) {
        check_slice_root_exists(self.slice_root_directory);
        match get_latest_slices_from_slice_root_directory(&self.slice_root_directory) {
            Ok(slices) => {
                let slices = parse_slices(slices);
                self.print_similar_slices_section(&slices);
                self.print_missing_dependencies_section(&slices);
                self.print_found_requested_slices_section(&slices);
                self.print_missing_requested_slices_section(&slices);
            }
            Err(error) => println!("{}", error),
        }
    }
}
