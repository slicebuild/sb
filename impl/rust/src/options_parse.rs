use std::env;
use std::str::FromStr;

#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
pub enum Format {
    Docker,
    Shell,
}

impl FromStr for Format {
    type Err = &'static str;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        match str {
            "d" => Ok(Format::Docker),
            "sh" => Ok(Format::Shell),
            _ => Err("Unknown format. Available formats = [d, sh]")
        }
    }
}

#[derive(Debug)]
pub struct Options {
    pub format: Format,
    pub outpath: String,
    pub url: String,
}

impl Options {
    pub fn new() -> Options {
        Options { format: Format::Shell, outpath: String::new(), url: String::new() }
    }

    fn get_format_option_name() -> &'static str {
        "f"
    }

    fn get_outpath_option_name() -> &'static str {
        "o"
    }

    fn get_url_option_name() -> &'static str {
        "url"
    }

    fn set_option(&mut self, option_name: String, option_value: String) -> Result<(), String> {
        let format_option_name = Options::get_format_option_name();
        let outpath_option_name = Options::get_outpath_option_name();
        let url_option_name = Options::get_url_option_name();

        match option_name {
            ref option_name if option_name == format_option_name => {
                match Format::from_str(&option_value) {
                    Ok(format) => self.format = format,
                    Err(error) => return Err(error.to_string())
                }
            }
            ref option_name if option_name == outpath_option_name => self.outpath = option_value,
            ref option_name if option_name == url_option_name => self.url = option_value,
            _ => return Err(format!("Unknown option = {}", option_name))
        }
        Ok(())
    }
}

pub fn parse_options() -> (String, Options, Vec<String>) {
    let mut options = Options::new();
    let mut args = env::args();
    let app_path = args.next().unwrap();

    let mut current_option_name: Option<String> = Option::None;
    let mut remaining_arguments: Vec<String> = Vec::new();
    for argument in args {
        if let Some(option_name) = current_option_name {
            let result = options.set_option(option_name.to_string(), argument);
            if let Err(error) = result {
                panic!("{}", error);
            }
            current_option_name = Option::None
        } else if argument.starts_with('-') {
            let argument_without_dashes = argument.trim_left_matches('-');
            if argument_without_dashes.contains('=') {
                let parts: Vec<&str> = argument_without_dashes.splitn(2, '=').collect();
                let option_name = parts.first().unwrap();
                let option_value = parts.last().unwrap();
                let result = options.set_option(option_name.to_string(), option_value.to_string());
                if let Err(error) = result {
                    panic!("{}", error);
                }
            } else {
                current_option_name = Some(argument_without_dashes.to_string());
            }
        } else {
            remaining_arguments.push(argument)
        }
    }
    (app_path, options, remaining_arguments)
}
