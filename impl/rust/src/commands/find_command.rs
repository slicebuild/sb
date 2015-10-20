use std::collections::BTreeSet;
use std::path::Path;
use super::check_slice_root_exists;
use super::command::Command;
use super::SectionKind;
use super::Slice;
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

    fn print_found_requested_slices_section(&self, similar_slices: &Vec<&Slice>) {
        let requested_slices = self.layers.iter().map(|layer| {
            similar_slices.iter().filter(|slice| slice.name == *layer).max()
        });
        let requested_slices = requested_slices.filter(|slice| slice.is_some());
        let requested_slices = requested_slices.map(|slice| slice.unwrap());
        let requested_slices = requested_slices.collect::<Vec<_>>();
        if requested_slices.len() == 0 {
            println!("Found requested: None");
        } else {
            println!("Found requested:");
            for slice in &requested_slices {
                println!("{}", slice.path.display());
            }
        }
    }

    fn print_missing_requested_slices_section(&self, visited_slices: &Vec<&str>) {
        let missing_requested_slices = self.layers.iter().filter(|layer| {
            !visited_slices.contains(layer)
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
                let mut missing_slice_dependencies = BTreeSet::<&str>::new();
                let mut visited_slices = Vec::<&str>::new();
                let mut similar_slices = Vec::<&Slice>::new();
                for slice in &slices {
                    visited_slices.push(&slice.name);
                    if self.is_slice_name_similar(&slice.name)
                    && self.is_slice_supports_os(slice) {
                        similar_slices.push(slice);
                    }
                    missing_slice_dependencies.remove(&*slice.name);
                    if let Some(dependencies) = slice.get_section_items(SectionKind::Dep) {
                        for dependency in dependencies {
                            if !visited_slices.contains(&dependency) {
                                missing_slice_dependencies.insert(dependency);
                            }
                        }
                    }
                }

                print_similar_slices_section(&similar_slices);
                print_missing_dependencies(&missing_slice_dependencies);
                self.print_found_requested_slices_section(&similar_slices);
                self.print_missing_requested_slices_section(&visited_slices);
            }
            Err(error) => println!("{}", error),
        }
    }
}

fn print_similar_slices_section(similar_slices: &Vec<&Slice>) {
    if similar_slices.len() == 0 {
        return println!("All similar: None");
    }

    println!("All similar:");
    for slice in similar_slices {
        println!("{}", slice.path.display());
    }
}

fn print_missing_dependencies(missing_dependencies: &BTreeSet<&str>) {
    if missing_dependencies.len() == 0 {
        return println!("All missing: None");
    }

    println!("All missing:");
    for slice in missing_dependencies {
        println!("{}", slice);
    }
}