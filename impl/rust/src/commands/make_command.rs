use std::borrow::Borrow;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use super::command::Command;
use options_parse::Options;
use formatters;
use slice::directory;
use slice::list::{DependentSlice, SliceList};

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

    fn create_code_from_slices(&self, slices: Vec<&DependentSlice>, visited_slices: &mut Vec<String>) -> String {
        let mut code = String::new();
        for slice in slices {
            let slice_content = slice.content();
            if visited_slices.contains(&slice_content.name) {
                continue;
            }
            let dependencies = slice.resolved_dependencies();
            if !dependencies.is_empty() {
                let dependency_code = self.create_code_from_slices(dependencies, visited_slices);
                code.push_str(&dependency_code);
            }
            let slice_code = formatters::code_for_slice(slice_content, self.options.format());
            code.push_str(&slice_code);
        }
        code
    }

    fn get_code_for_latest_slice(&self) -> Result<String, String> {
        match directory::get_latest_slices_from_slice_root_directory(&self.slice_root_directory) {
            Ok(slices) => {
                let slice_list = SliceList::from_slices(slices);
                let slice_list = slice_list.filter_for_os(self.os);
                let mut unresolved_slices = Vec::new();
                let mut resolved_slices = Vec::new();
                for layer in self.layers {
                    if let Some(slice) = slice_list.find_slice(layer, None) {
                        if slice.has_unresolved_dependencies() {
                            for slice in slice.unresolved_dependencies() {
                                unresolved_slices.push(slice.clone());
                            }
                        } else {
                            resolved_slices.push(slice.clone());
                        }
                    } else {
                        unresolved_slices.push(layer.to_string());
                    }
                }
                if !unresolved_slices.is_empty() {
                    println!("Missing slices:");
                    for slice in unresolved_slices {
                        println!("{}", slice);
                    }
                    return Err("Has missing slices".to_string());
                }
                let slices = resolved_slices.iter().map(|slice| slice.borrow()).collect::<Vec<_>>();
                let mut visited_slices = Vec::new();
                Ok(self.create_code_from_slices(slices, &mut visited_slices))
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
