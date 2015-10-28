extern crate zip;
use std::fs::{create_dir, create_dir_all, File};
use std::io::{Cursor, Read, Write};
use std::path::{Path, PathBuf};
use curl::http;
use rustc_serialize::json::Json;
use zip::read::ZipArchive;
use version;
use super::command::Command;

pub struct FetchCommand<'a> {
    slice_root_directory: &'a Path,
}

/// # Panics
/// Panics for empty versions
fn choose_latest_version<'a>(versions: &'a Vec<&str>) -> &'a str {
    assert_not_empty!(versions);
    let iter = versions.iter().enumerate();
    let (_, i) = iter.map(|(i, v)| {
        let (_, version) = version::extract_name_and_version(v);
        (version, i)
                     })
                     .max()
                     .unwrap();
    versions.iter().nth(i).unwrap()
}

impl<'a> FetchCommand<'a> {
    /// # Examples
    /// ```
    /// use sb::commands::fetch_command::FetchCommand;
    /// use std::path::Path;
    /// let command = FetchCommand::new(Path::new("/tmp"));
    /// ```
    pub fn new(slice_root_directory: &'a Path) -> FetchCommand {
        FetchCommand { slice_root_directory: slice_root_directory }
    }

    fn execute_request_to_uri(uri: &str) -> Vec<u8> {
        let mut handle = http::handle();
        let request = handle.get(uri).header("user-agent", "Mozilla/4.0 (compatible)");
        let response = request.exec().unwrap();
        response.move_body()
    }

    fn determine_latest_version() -> String {
        let body = FetchCommand::execute_request_to_uri("https://api.github.\
                                                         com/repos/slicebuild/slices/branches");
        let body = String::from_utf8(body).unwrap();
        let json = Json::from_str(&body).unwrap();
        let array = json.as_array().unwrap();
        let iter = array.into_iter();
        let versions = iter.map(|item| {
            match item.as_object() {
                Some(obj) => {
                    let field = "name".to_string();
                    match obj.get(&field) {
                        Some(name) => {
                            match name.as_string() {
                                Some(name) => name,
                                None => panic!("{} is not a string", field)
                            }
                        },
                        None => panic!("Object has no \"{}\" field", field)
                    }
                }
                None => panic!("Expected object, but received {:?}", item)
            }
                           })
                           .filter(|name| *name != "master")
                           .collect::<Vec<_>>();
        choose_latest_version(&versions).to_string()
    }

    fn download_latest_version() -> Vec<u8> {
        let version = FetchCommand::determine_latest_version();
        println!("Version = {}", version);
        let uri = format!("https://codeload.github.com/slicebuild/slices/zip/{}",
                          version);
        FetchCommand::execute_request_to_uri(&uri)
    }

    fn extract_archive_into_directory(mut zip_archive: ZipArchive<Cursor<Vec<u8>>>,
                                      path: PathBuf) {
        for i in 0..zip_archive.len() {
            let mut path = path.clone();
            let mut file = zip_archive.by_index(i).unwrap();
            {
                let file_name = file.name();
                path.push(file_name);
            }
            if file.size() != 0 {
                let _ = create_dir_all(path.parent().unwrap());
                let mut bytes: Vec<u8> = Vec::with_capacity(file.size() as usize);
                let _ = file.read_to_end(&mut bytes).unwrap();
                FetchCommand::write_bytes_to_temporary_file(&bytes, &path);
            }
        }
    }

    fn write_bytes_to_temporary_file(bytes: &[u8], file_path: &PathBuf) {
        let _ = create_dir(file_path.parent().unwrap());
        let mut file = File::create(file_path).unwrap();
        let _ = file.write(bytes);
    }
}

impl<'a> Command for FetchCommand<'a> {
    fn run(&mut self) {
        let bytes = FetchCommand::download_latest_version();
        let cursor = Cursor::new(bytes);
        let zip_archive = ZipArchive::new(cursor).unwrap();
        let directory = self.slice_root_directory.to_path_buf();
        FetchCommand::extract_archive_into_directory(zip_archive, directory);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn choose_latest_version() {
        let versions = vec!["du-0.0.1-rc.1",
                            "du-0.0.2",
                            "my-du-1.0.0",
                            "ubuntu-only-1.0.1",
                            "fed-2.1.1"];
        assert_eq!(super::choose_latest_version(&versions), "fed-2.1.1");
    }
}
