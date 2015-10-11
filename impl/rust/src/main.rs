extern crate sb;
//use sb::options_parse::parse_options;
use sb::commands::command::Command;
//use sb::commands::fetch_command::FetchCommand;
use sb::commands::find_command::FindCommand;
use std::path::Path;

fn main() {
//    let (options, mut arguments) = parse_options();
//    assert!(arguments.len() != 0);
//    let command = arguments.remove(0);

    let path = Path::new("/home/owl/sb/impl/rust/test_slices");
    let mut command = FindCommand { os: "debian".to_string(), layer: "jekyll".to_string(), slices_directory: path };
//    let mut command = FetchCommand { slices_directory: "/tmp/slices".to_string() };
    command.run();
}
