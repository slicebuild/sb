extern crate sb;
use sb::options_parse::parse_options;
use sb::commands::command::Command;
use sb::commands::fetch_command::FetchCommand;
use sb::commands::find_command::FindCommand;
use sb::commands::make_command::MakeCommand;
use std::env::current_dir;
use std::path::{Path, PathBuf};

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
            assert!(arguments.len() == 2);
            let os = arguments.remove(0);
            let layer = arguments.remove(0);
            run_command(FindCommand {
                layer: layer,
                os: os,
                slice_root_directory: slice_root_directory,
            })
        }
        "fetch" => {
            run_command(FetchCommand { slice_root_directory: slice_root_directory })
        }
        "make" => {
            assert!(arguments.len() == 2);
            let os = arguments.remove(0);
            let layer = arguments.remove(0);
            run_command(MakeCommand {
                layer: layer,
                os: os,
                slice_root_directory: slice_root_directory,
            })
        }
        _ => panic!(),
    };
}

fn run_command<T>(mut command: T)
    where T: Command
{
    command.run()
}

fn get_slice_root_directory(app_path: &String) -> PathBuf {
    let mut slice_root_directory = PathBuf::new();
    slice_root_directory.push(Path::new(&app_path).parent().unwrap());
    slice_root_directory.push(".sb");
    slice_root_directory.push("slices");
    slice_root_directory
}
