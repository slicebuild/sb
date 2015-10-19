use std::path::Path;
use super::helper::check_slice_root_exists;
use super::command::Command;
use super::super::slice::directory::get_latest_slices_from_slice_root_directory;

pub struct FindCommand<'a> {
    pub layers: Vec<String>,
    pub os: String,
    pub slice_root_directory: &'a Path,
}

impl<'a> Command for FindCommand<'a> {
    fn run(&mut self) {
        check_slice_root_exists(self.slice_root_directory);
        match get_latest_slices_from_slice_root_directory(&self.slice_root_directory) {
            Ok(slices) => {
                let slices = slices.into_iter().filter(|slice| {
                    let os_list = slice.get_os_list();
                    os_list.contains(&self.os) && self.layers.iter().any(|layer| slice.name.contains(layer))
                });
                for slice in slices {
                    if let Some(ref version) = slice.version {
                        println!("{}-{}", slice.name, version);
                    } else {
                        println!("{}", slice.name);
                    }
                }
            }
            Err(error) => println!("{}", error),
        }
    }
}
