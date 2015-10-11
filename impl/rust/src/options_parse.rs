use std::cell::RefCell;
use std::env;

#[derive(Debug)]
pub struct Options {
    format: String,
    outdir: String,
    url: String
}

impl Options {
    fn get_format_option_name() -> &'static str {
        "f"
    }

    fn get_outdir_option_name() -> &'static str {
        "o"
    }

    fn get_url_option_name() -> &'static str {
        "url"
    }

    fn has_option_with_name(option_name: &String) -> bool {
        let format_option_name = Options::get_format_option_name();
        let outdir_option_name = Options::get_outdir_option_name();
        let url_option_name = Options::get_url_option_name();

        option_name == format_option_name || option_name == outdir_option_name || option_name == url_option_name
    }

    fn set_option(&mut self, option_name: String, option_value: String) {
        let format_option_name = Options::get_format_option_name();
        let outdir_option_name = Options::get_outdir_option_name();
        let url_option_name = Options::get_url_option_name();

        if option_name == format_option_name {
            self.format = option_value
        } else if option_name == outdir_option_name {
            self.outdir = option_value
        } else if option_name == url_option_name {
            self.url = option_value
        } else {
            println!("Unknown option = {}", option_name)
        }
    }
}

pub fn parse_options() -> (String, Options, Vec<String>) {
    let options = Options { format: String::new(), outdir: String::new(), url: String::new() };
    let options = RefCell::new(options);

    let mut args = env::args();
    let app_path = args.next().unwrap();

    let mut current_option_name: Option<String> = Option::None;
    let mut remaining_arguments: Vec<String> = Vec::new();
    for argument in args {
        if let Some(option_name) = current_option_name {
            println!("Current option = {}", option_name);
            println!("Option value = {}", argument);
            let mut options = options.borrow_mut();
            options.set_option(option_name.to_string(), argument);
            current_option_name = Option::None
        } else if argument.starts_with('-') {
            let argument_without_dashes = argument.trim_left_matches('-');
            if argument_without_dashes.contains('=') {
                let parts: Vec<&str> = argument_without_dashes.splitn(2, '=').collect();
                let option_name = parts.first().unwrap();
                println!("Option name = {}", option_name);
                println!("Option contains value after '='");
                let option_value = parts.last().unwrap();
                println!("Option value = {}", option_value);
                let mut options = options.borrow_mut();
                options.set_option(option_name.to_string(), option_value.to_string());
            } else {
                let option_name = argument_without_dashes.to_string();
                if Options::has_option_with_name(&option_name) {
                    current_option_name = Some(option_name);
                } else {
                    println!("Unknown option {}", argument_without_dashes)
                }
            }
        } else {
            remaining_arguments.push(argument)
        }
    }

    (app_path, options.into_inner(), remaining_arguments)
}
