use std::path::Path;
use super::command::Command;
#[cfg(test)]
use super::super::for_testing::get_slice_root_directory;
use super::super::slice::item::Slice;
use super::super::slice::directory::get_latest_slices_from_slice_root_directory;
use super::super::slice::section::Kind;

pub struct MakeCommand<'a> {
    pub layer: String,
    pub os: String,
    pub slice_root_directory: &'a Path,
}

impl<'a> MakeCommand<'a> {
    fn find_main_slice(&self, slices: &'a Vec<Slice>) -> &'a Slice {
        slices.iter()
              .find(|slice| {
                  slice.name.contains(&self.layer) && slice.get_os_list().contains(&self.os)
              })
              .unwrap()
    }

    fn add_code_for_slice(current_code: &mut String,
                          slice: &Slice,
                          available_slices: &mut Vec<&Slice>) {
        if let Some(dep_section) = slice.sections.iter().find(|section| section.kind == Kind::Dep) {
            for dependency in &dep_section.items {
                let dependency_position = available_slices.iter().position(|slice| {
                    slice.name == *dependency
                });
                if let Some(dependency_position) = dependency_position {
                    let dependency = available_slices.remove(dependency_position);
                    MakeCommand::add_code_for_slice(current_code, &dependency, available_slices);
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
            Ok(slices) => {
                let mut available_slices: Vec<&Slice> = Vec::new();
                for slice in &slices {
                    available_slices.push(&slice);
                }
                let main_layer = self.find_main_slice(&slices);
                let mut string = String::new();
                MakeCommand::add_code_for_slice(&mut string, &main_layer, &mut available_slices);
                Ok(string)
            }
            Err(error) => Err(error),
        }
    }
}

impl<'a> Command for MakeCommand<'a> {
    fn run(&mut self) {
        match self.get_code_for_latest_slice() {
            Ok(code) => println!("Code = {}", code),
            Err(error) => println!("Error = {}", error),
        }
    }
}

#[test]
fn test_make_command() {
    let expected_code = "apt-get update -y
apt-get install libc6-dev libssl-dev make \
                         build-essential libssl-dev libreadline6-dev zlib1g-dev libyaml-dev \
                         libz-dev -y
apt-get upgrade -y
apt-get install wget -y
cd /tmp
wget \
                         https://cache.ruby-lang.org/pub/ruby/2.2/ruby-2.2.3.tar.gz
tar xvzf \
                         ruby-2.2.3.tar.gz
cd ruby-2.2.3
./configure --prefix=/usr/local
make
\
                         make install
cd ..
wget https://rubygems.org/rubygems/rubygems-2.4.8.tgz
\
                         tar xvzf rubygems-2.4.8.tgz
cd rubygems-2.4.8
ruby setup.rb
gem install \
                         jekyll -v '3.0.0.pre.beta9'
";
    let slice_root_directory = get_slice_root_directory();
    let command = MakeCommand {
        layer: "jekyll".to_string(),
        os: "debian".to_string(),
        slice_root_directory: &slice_root_directory,
    };
    let returned_code = command.get_code_for_latest_slice().unwrap();
    assert_eq!(returned_code, expected_code);
}
