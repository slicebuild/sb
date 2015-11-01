use std::borrow::Borrow;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use {RequestedSlice, VersionMatchStrategy};
use commands::command::Command;
use formatters;
use options_parse::Options;
use os::Os;
use slice::{List, Slice};

pub struct MakeCommand<'a> {
    slices: Vec<RequestedSlice>,
    os: Os,
    root_directory: &'a Path,
    slice_root_directory: &'a Path,
    options: Options
}

struct DependenciesSearchResult<'a> {
    found_requested_slices: Vec<&'a Slice>,
    missing_requested_slices: Vec<&'a str>,
    unresolved_dependencies: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> MakeCommand<'a> {
    /// # Panics
    /// If `slices` is empty
    pub fn new(slices: Vec<RequestedSlice>, os: Os, root_directory: &'a Path,
           slice_root_directory: &'a Path, options: Options) -> MakeCommand<'a> {
        assert_not_empty!(slices);
        assert!(!slices.is_empty(), "There is no specified layers");
        MakeCommand { slices: slices, os: os, root_directory: root_directory,
                      slice_root_directory: slice_root_directory,
                      options: options }
    }

    fn create_code_from_slices(&self, slices: Vec<&Slice>, visited_slices: &mut Vec<&str>) -> String {
        let mut code = String::new();
        for slice in slices {
            let slice_name = slice.name() as &str;
            if visited_slices.contains(&slice_name) {
                continue;
            }
            let dependencies = slice.resolved_dependencies();
            if !dependencies.is_empty() {
                let dependency_code = self.create_code_from_slices(dependencies, visited_slices);
                code.push_str(&dependency_code);
            }
            let slice_code = formatters::generate_code(&slice, &self.options.format);
            code.push_str(&slice_code);
        }
        code
    }

    fn get_output_file_path(&self) -> PathBuf {
        if self.options.outpath.is_empty() {
            let mut path = self.root_directory.to_path_buf();
            path.push("make");
            path.push(&self.slices.first().unwrap().name);
            path
        } else {
            let mut path = PathBuf::new();
            path.push(&self.options.outpath);
            path
        }
    }

    fn find_unresolved_dependencies<'b>(&'b self, list: &'b List) -> DependenciesSearchResult {
        let mut result = DependenciesSearchResult { found_requested_slices: Vec::new(),
                                                    missing_requested_slices: Vec::new(),
                                                    unresolved_dependencies: HashMap::new() };
        for slice in &self.slices {
            if let Some(slice) = list.find_slice(&slice.name, &slice.version,
                                                 slice.version_match_strategy) {
                let dependencies = slice.unresolved_dependencies_include_nested();
                if !dependencies.is_empty() {
                    for dep in dependencies {
                        let entry = result.unresolved_dependencies.entry(&slice.name());
                        entry.or_insert(Vec::new()).push(dep);
                    }
                } else {
                    result.found_requested_slices.push(slice.borrow());
                }
            } else {
                result.missing_requested_slices.push(&slice.name);
            }
        }
        result
    }

    fn generate_code(&self) -> String {
        match List::new(&self.slice_root_directory, &self.os,
                        VersionMatchStrategy::ExactOrGreater) {
            Ok(list) => {
                match self.generate_code_for_list(&list) {
                    Some(code) => code,
                    None => panic!("Code generation failed")
                }
            }
            Err(error) => panic!("{}", error)
        }
    }

    fn generate_code_for_list(&self, list: &List) -> Option<String> {
        let result = self.find_unresolved_dependencies(list);
        let has_missing_slice = !result.unresolved_dependencies.is_empty() ||
                                !result.missing_requested_slices.is_empty();
        if has_missing_slice {
            if !result.missing_requested_slices.is_empty() {
                println!("Missing requested slices:");
                for slice in &result.missing_requested_slices {
                    println!("{}", slice);
                }
            }
            if !result.unresolved_dependencies.is_empty() {
                println!("Missing dependencies:");
                for (requested_slice, missing_dependencies) in &result.unresolved_dependencies {
                    for missing_dep in missing_dependencies {
                        println!("{} depends on {}, but it is missing", requested_slice, missing_dep);
                    }
                }
            }
            return None;
        }

        let mut visited_slices = Vec::new();
        Some(self.create_code_from_slices(result.found_requested_slices, &mut visited_slices))
    }

    fn write_code(&self, code: String) {
        match File::create(self.get_output_file_path()) {
            Ok(mut file) => {
                if let Err(error) = file.write_fmt(format_args!("{}", code)) {
                    panic!("{}", error);
                }
            }
            Err(error) =>  panic!("File cannot be created at path {}\nReason = {}",
                                  self.get_output_file_path().display(), error)
        }
    }
}

impl<'a> Command for MakeCommand<'a> {
    fn run(&mut self) {
        let code = self.generate_code();
        self.write_code(code);
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::path::Path;
    use {MakeCommand, RequestedSlice, VersionMatchStrategy};
    use options_parse::{Format, Options};
    use os::Os;
    use version;

    fn generate_code_for_jekyll_with_format(format: Format) -> String {
        let version_match_strategy = VersionMatchStrategy::ExactOrGreater;
        let slice = RequestedSlice { name: String::from("jekyll"), version: version::zero(),
                                     version_match_strategy: version_match_strategy };
        let slices = vec![slice];

        let os = Os { name: String::from("debian"), version: version::zero() };

        let mut path = env::current_dir().expect("Current dir is not set").to_path_buf();
        path.push("test_slices");

        let options = Options { format: format, .. Options::new() };

        let command = MakeCommand::new(slices, os, Path::new("/"), &path, options);
        println!("{}", command.generate_code());
        command.generate_code()
    }

    #[test]
    fn generate_code_for_jekyll_in_docker_format() {
        let code = generate_code_for_jekyll_with_format(Format::Docker);
        assert_eq!(code, "RUN export DEBIAN_FRONTEND=noninteractive && \\
apt-get update -q -y && \\
alias apt-get='apt-get --no-install-recommends' && \\
apt-get install -q -y apt-utils
RUN apt-get install -q -y wget
RUN cd /tmp && \\
wget https://cache.ruby-lang.org/pub/ruby/2.2/ruby-2.2.3.tar.gz && \\
tar xvzf ruby-2.2.3.tar.gz && \\
cd ruby-2.2.3 && \\
./configure --prefix=/usr/local && \\
make && \\
make install && \\
cd .. && \\
rm -rf ruby-2.2.3 && \\
wget https://rubygems.org/rubygems/rubygems-2.4.8.tgz && \\
tar xvzf rubygems-2.4.8.tgz && \\
cd rubygems-2.4.8 && \\
ruby setup.rb && \\
cd .. && \\
rm -rf rubygems-2.4.8
RUN gem install jekyll -v '3.0.0.pre.beta9'
");
    }

    #[test]
    fn generate_code_for_jekyll_in_shell_format() {
        let code = generate_code_for_jekyll_with_format(Format::Shell);
        assert_eq!(code, "export DEBIAN_FRONTEND=noninteractive
apt-get update -q -y
alias apt-get='apt-get --no-install-recommends'
apt-get install -q -y apt-utils
apt-get install -q -y wget
cd /tmp
wget https://cache.ruby-lang.org/pub/ruby/2.2/ruby-2.2.3.tar.gz
tar xvzf ruby-2.2.3.tar.gz
cd ruby-2.2.3
./configure --prefix=/usr/local
make
make install
cd ..
rm -rf ruby-2.2.3
wget https://rubygems.org/rubygems/rubygems-2.4.8.tgz
tar xvzf rubygems-2.4.8.tgz
cd rubygems-2.4.8
ruby setup.rb
cd ..
rm -rf rubygems-2.4.8
gem install jekyll -v '3.0.0.pre.beta9'
");
    }
}