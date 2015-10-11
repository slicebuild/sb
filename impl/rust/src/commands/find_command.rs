use std::fs::metadata;
use std::io::ErrorKind;
use std::path::Path;
use super::command::Command;
use super::super::slice::item::Slice;
use super::super::slice::directory::{get_latest_slices_from_slice_root_directory};

pub struct FindCommand<'a> {
    pub layer: String,
    pub os: String,
    pub slice_root_directory: &'a Path
}

impl<'a> Command for FindCommand<'a> {
    fn run(&mut self) {
        match metadata(self.slice_root_directory) {
            Ok(_) => {
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
            },
            Err(error) => match error.kind() {
                ErrorKind::NotFound => println!("Slice root directory is not exists. Path = {}", self.slice_root_directory.to_str().unwrap()),
                _ => println!("Unknown error = {}", error)
            }
        }
    }
}
