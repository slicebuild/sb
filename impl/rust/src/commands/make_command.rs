use std::cell::RefCell;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use super::{DependentSlice, parse_slices};
use super::command::Command;
use super::super::options_parse::Options;
use super::super::slice::Slice;
use super::super::slice::directory::get_latest_slices_from_slice_root_directory;
use super::super::slice::section::Kind;

pub struct MakeCommand<'a> {
    layers: &'a Vec<&'a str>,
    os: &'a str,
    root_directory: &'a Path,
    slice_root_directory: &'a Path,
    options: Options
}

impl<'a> MakeCommand<'a> {
    pub fn new(layers: &'a Vec<&'a str>, os: &'a str, root_directory: &'a Path,
           slice_root_directory: &'a Path, options: Options) -> MakeCommand<'a> {
        assert!(!layers.is_empty(), "There is no specified layers");
        assert!(!os.is_empty(), "There is no specified os");
        MakeCommand { layers: layers, os: os, root_directory: root_directory,
                      slice_root_directory: slice_root_directory,
                      options: options }
    }

    fn add_code_for_slice(&self, slice: Rc<RefCell<DependentSlice>>,
                          current_code: &mut String,
                          visited_slices: &mut Vec<String>) {
        let slice = slice.borrow();
        {
            let os_list = slice.slice.get_os_list();
            assert!(os_list.contains(&self.os), "Slice \"{}\" does not support os \"{}\"",
                    slice.slice.name, self.os);
        }
        for dependency in &slice.resolved_dependencies {
            let dependency = dependency.clone();
            let dependency_added_before = {
                let dependency = dependency.borrow();
                if visited_slices.contains(&dependency.slice.name) {
                    true
                } else {
                    visited_slices.push(dependency.slice.name.clone());
                    false
                }
            };
            if !dependency_added_before {
                self.add_code_for_slice(dependency, current_code, visited_slices);
            }
        }
        for item in slice.slice.get_run_list() {
            current_code.push_str(&item);
            current_code.push('\n');
        }
    }

    fn get_code_for_latest_slice(&self) -> Result<String, String> {
        match get_latest_slices_from_slice_root_directory(&self.slice_root_directory) {
            Ok(slices) => {
                let slices = parse_slices(slices);
                let mut requested_slices = Vec::new();
                let mut missing_requested_slices = Vec::new();
                let mut missing_dependencies = Vec::new();
                for layer in self.layers {
                    let slice = slices.iter().find(|slice| {
                        let slice = slice.clone();
                        let slice = slice.borrow();
                        slice.slice.name == *layer
                    });
                    if let Some(slice) = slice {
                        let slice_content = slice.clone();
                        let slice_content = slice_content.borrow();
                        let unresolved_dependencies = slice_content.unresolved_dependencies();
                        if unresolved_dependencies.is_empty() {
                            requested_slices.push(slice);
                        } else {
                            for dependency in unresolved_dependencies {
                                missing_dependencies.push(dependency);
                            }
                        }
                    } else {
                        missing_requested_slices.push(layer.to_string());
                    }
                }

                let mut has_missing_slice = false;
                if !missing_requested_slices.is_empty() {
                    has_missing_slice = true;
                    println!("Missing requested slices:");
                    for slice in missing_requested_slices {
                        println!("{}", slice);
                    }
                }
                if !missing_dependencies.is_empty() {
                    has_missing_slice = true;
                    println!("Missing dependencies:");
                    for dependency in missing_dependencies {
                        println!("{}", dependency);
                    }
                }

                if has_missing_slice {
                    Err("Has missing dependencies".to_string())
                } else {
                    let mut visited_slices = Vec::new();
                    let mut string = String::new();
                    for slice in requested_slices {
                        self.add_code_for_slice(slice.clone(), &mut string, &mut visited_slices);
                    }
                    Ok(string)
                }
            }
            Err(error) => Err(error)
        }
    }

    fn get_output_file_path(&self) -> PathBuf {
        if self.options.outpath.is_empty() {
            let mut path = self.root_directory.to_path_buf();
            path.push(&self.layers.first().unwrap());
            path
        } else {
            let mut path = PathBuf::new();
            path.push(&self.options.outpath);
            path
        }
    }
}

impl<'a> Command for MakeCommand<'a> {
    fn run(&mut self) {
        match self.get_code_for_latest_slice() {
            Ok(code) => {
                let path = self.get_output_file_path();
                let path_as_string = path.to_str().unwrap().to_string();
                if let Ok(mut file) = File::create(path) {
                    file.write_fmt(format_args!("{}", &code)).unwrap();
                } else {
                    panic!("File cannot be created at path {}", &path_as_string);
                }
            }
            Err(error) => panic!("{}", error)
        }
    }
}
