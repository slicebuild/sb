use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
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

    fn add_code_for_slice_with_name(&self, slice_name: &str,
                                    current_code: &mut String,
                                    available_slices: &mut Vec<Slice>) {
        if let Some(slice_position) = available_slices.iter().position(|slice| slice.name == *slice_name) {
            let slice = available_slices.remove(slice_position);
            self.add_code_for_slice(&slice, current_code, available_slices);
        }
    }

    fn add_code_for_slice(&self, slice: &Slice,
                          current_code: &mut String,
                          available_slices: &mut Vec<Slice>) {
        {
            let os_list = slice.get_os_list();
            assert!(os_list.contains(&self.os), "Slice \"{}\" does not support os \"{}\"",
                    slice.name, self.os);
        }
        if let Some(dep_section) = slice.sections.iter().find(|section| section.kind == Kind::Dep) {
            for dependency in &dep_section.items {
                let dependency_position = available_slices.iter().position(|slice| {
                    slice.name == *dependency
                });
                if let Some(dependency_position) = dependency_position {
                    let dependency = available_slices.remove(dependency_position);
                    self.add_code_for_slice(&dependency, current_code, available_slices);
                }
            }
        }
        let run_section = slice.sections.iter().find(|section| section.kind == Kind::Run).unwrap();
        for item in &run_section.items {
            current_code.push_str(&item);
            current_code.push('\n');
        }
    }

    fn get_code_for_latest_slice(&self) -> Result<String, String> {
        match get_latest_slices_from_slice_root_directory(&self.slice_root_directory) {
            Ok(mut slices) => {
                let mut string = String::new();
                for layer in self.layers {
                    self.add_code_for_slice_with_name(layer, &mut string, &mut slices);
                }
                Ok(string)
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
