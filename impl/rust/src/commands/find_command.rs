use std::path::Path;
use super::command::Command;
use super::super::slice::item::Slice;
use super::super::slice::directory::{get_latest_slices_from_slice_root_directory};

pub struct FindCommand<'a> {
    pub layer: String,
    pub os: String,
    pub slices_directory: &'a Path
}

impl<'a> Command for FindCommand<'a> {
    fn run(&mut self) {
        let slices = get_latest_slices_from_slice_root_directory(&self.slices_directory);
        let slices = slices.into_iter().filter(|slice| {
            let os_list = slice.get_os_list();
            os_list.contains(&self.os) && slice.name.contains(&self.layer)
        });
        for slice in slices {
            println!("{}", slice.name);
        }
        //println!("Slice count = {}", slices.len());
        //println!("Slices = {:?}", slices);
    }
}
