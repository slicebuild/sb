use std::env::current_dir;
use std::fs::metadata;
use std::io::ErrorKind;
use std::path::Path;
use super::helper::check_slice_root_exists;
use super::command::Command;
use super::super::slice::directory::{get_latest_slices_from_slice_root_directory};
#[cfg(test)]
use super::super::for_testing::get_slice_root_directory;

pub struct FindCommand<'a> {
    pub layer: String,
    pub os: String,
    pub slice_root_directory: &'a Path
}

impl<'a> Command for FindCommand<'a> {
    fn run(&mut self) {
        check_slice_root_exists(self.slice_root_directory);
        match get_latest_slices_from_slice_root_directory(&self.slice_root_directory) {
            Ok(slices) => {
                let slices = slices.into_iter().filter(|slice| {
                    let os_list = slice.get_os_list();
                    os_list.contains(&self.os) && slice.name.contains(&self.layer)
                });
                for slice in slices {
                    println!("{}", slice.name);
                }
            },
            Err(error) => println!("{}", error)
        }
    }
}

#[test]
fn test_find_command_run() {
    let slice_root_directory = get_slice_root_directory();
    let mut command = FindCommand { layer: "jekyll".to_string(), os: "debian".to_string(), slice_root_directory: &slice_root_directory };
    command.run();
}