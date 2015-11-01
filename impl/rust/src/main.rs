extern crate sb;
use std::env::current_dir;
use std::path::PathBuf;
use sb::{RequestedSlice, VersionMatchStrategy};
use sb::options_parse::{Options, parse_options};
use sb::commands::{Command, FetchCommand, FindCommand, MakeCommand};
use sb::os::Os;
use sb::version;

const DEFAULT_OS: &'static str = "debian";
const DEFAULT_LAYER: &'static str = "jekyll";

fn main() {
    let (app_path, options, mut arguments) = parse_options();
    if arguments.is_empty() {
        panic!("Command expected")
    }
    let command = arguments.remove(0);
    let command: &str = &command;
    match command {
        "find" => run_find_command(app_path, arguments),
        "fetch" => run_fetch_command(app_path),
        "make" => run_make_command(app_path, arguments, options),
        _ => panic!("Unknown command \"{}\"", command)
    }
}

fn pop_first_argument_or_take_default(arguments: &mut Vec<String>, default_value: String) -> String {
    if arguments.len() == 0 {
        default_value
    } else {
        arguments.remove(0)
    }
}

fn get_os_from_arguments_or_default(arguments: &mut Vec<String>) -> Os {
    let os = pop_first_argument_or_take_default(arguments, DEFAULT_OS.to_string());
    let (name, version) = version::extract_name_and_version(&os);
    Os { name: name, version: version }
}

fn get_layers_from_arguments_or_default(arguments: &mut Vec<String>) -> Vec<RequestedSlice> {
    let argument = pop_first_argument_or_take_default(arguments, DEFAULT_LAYER.to_string());
    argument.split(',')
            .map(|slice| {
                let (name, version) = version::extract_name_and_version(slice);
                RequestedSlice { name: name, version: version, version_match_strategy: VersionMatchStrategy::ExactOrGreater }
            })
            .collect()
}

fn run_fetch_command(app_path: String) {
    let root_directory = get_root_directory(&app_path);
    let slice_root_directory = get_slice_root_directory(&root_directory);
    let mut command = FetchCommand::new(&slice_root_directory);
    command.run();
}

fn run_find_command(app_path: String, mut arguments: Vec<String>) {
    let layers = get_layers_from_arguments_or_default(&mut arguments);
    let os = get_os_from_arguments_or_default(&mut arguments);
    let root_directory = get_root_directory(&app_path);
    let slice_root_directory = get_slice_root_directory(&root_directory);
    let mut command = FindCommand::new(layers, os, &slice_root_directory);
    command.run();
}

fn run_make_command(app_path: String, mut arguments: Vec<String>, options: Options) {
    let layers = get_layers_from_arguments_or_default(&mut arguments);
    let os = get_os_from_arguments_or_default(&mut arguments);
    let root_directory = get_root_directory(&app_path);
    let slice_root_directory = get_slice_root_directory(&root_directory);
    let mut command = MakeCommand::new(layers, os, &root_directory,
                                       &slice_root_directory, options);
    command.run();
}

fn get_root_directory(app_path: &str) -> PathBuf {
    let mut root_directory = PathBuf::new();
    root_directory.push(app_path);
    if root_directory.is_relative() {
        let mut new_root_directory = current_dir().unwrap();
        new_root_directory.push(root_directory);
        root_directory = new_root_directory;
    }
    assert_eq!(root_directory.pop(), true);
    root_directory.push(".sb");
    root_directory
}

fn get_slice_root_directory(root_directory: &PathBuf) -> PathBuf {
    let mut slice_root_directory = root_directory.clone();
    slice_root_directory.push("slices");
    slice_root_directory
}
