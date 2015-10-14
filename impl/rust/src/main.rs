extern crate sb;
use sb::options_parse::parse_options;
use sb::commands::command::Command;
use sb::commands::fetch_command::FetchCommand;
use sb::commands::find_command::FindCommand;
use sb::commands::make_command::MakeCommand;
use std::env::current_dir;
use std::path::{Path, PathBuf};

const DEFAULT_OS: &'static str = "debian";
const DEFAULT_LAYER: &'static str = "jekyll";

fn main() {
    let (app_path, options, mut arguments) = parse_options();

    let mut slice_root_directory = get_slice_root_directory(&app_path);
    if slice_root_directory.is_relative() {
        let mut new_slice_root_directory = current_dir().unwrap();
        new_slice_root_directory.push(slice_root_directory);
        slice_root_directory = new_slice_root_directory;
    }
    let slice_root_directory = &*slice_root_directory;

    assert!(arguments.len() != 0);
    let command = arguments.remove(0);
    let command: &str = &command;
    match command {
        "find" => {
            let os = get_os_from_arguments_or_default(&mut arguments);
            let layer = get_layer_from_arguments_or_default(&mut arguments);
            run_command(FindCommand { layer: layer, os: os, slice_root_directory: slice_root_directory })
        },
        "fetch" => {
            run_command(FetchCommand { slice_root_directory: slice_root_directory })
        },
        "make" => {
            let os = get_os_from_arguments_or_default(&mut arguments);
            let layer = get_layer_from_arguments_or_default(&mut arguments);
            run_command(MakeCommand { layer: layer, os: os, slice_root_directory: slice_root_directory })
        },
        _ => panic!()
    };
}

fn run_command<T>(mut command: T) where T : Command {
    command.run()
}

fn pop_first_argument_or_take_default(arguments: &mut Vec<String>, default_value: String) -> String {
    if arguments.len() == 0 {
        default_value
    } else {
        arguments.remove(0)
    }
}

fn get_os_from_arguments_or_default(arguments: &mut Vec<String>) -> String {
    pop_first_argument_or_take_default(arguments, DEFAULT_OS.to_string())
}

fn get_layer_from_arguments_or_default(arguments: &mut Vec<String>) -> String {
    pop_first_argument_or_take_default(arguments, DEFAULT_LAYER.to_string())
}

fn get_slice_root_directory(app_path: & String) -> PathBuf {
    let mut slice_root_directory = PathBuf::new();
    slice_root_directory.push(Path::new(&app_path).parent().unwrap());
    slice_root_directory.push(".sb");
    slice_root_directory.push("slices");
    slice_root_directory
}
