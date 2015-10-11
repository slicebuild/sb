extern crate zip;
use curl::http;
use rustc_serialize::json::Json;
use semver::Version;
use std::fs::{create_dir, create_dir_all, File};
use std::io::{Cursor, Read, Write};
use std::path::{Path, PathBuf};
use zip::read::ZipArchive;
use super::command::Command;

pub struct FetchCommand {
    pub slices_directory: String
}

impl FetchCommand {
    fn choose_latest_version(array: &Vec<Json>) -> Version {
        let mut latest_version: Option<Version> = None;
        assert!(!array.is_empty());
        for item in array {
            let object = item.as_object().unwrap();
            let name = object.get("name").unwrap();
            let name = name.as_string().unwrap();
            if name == "master" {
                continue;
            }

            let version = Version::parse(&name).unwrap();
            if let Some(some_latest_version) = latest_version {
                if some_latest_version < version {
                    latest_version = Some(version);
                } else {
                    latest_version = Some(some_latest_version);
                }
            } else {
                latest_version = Some(version);
            }
        }
        latest_version.unwrap()
    }

    fn execute_request_to_uri(uri: &str) -> Vec<u8> {
        let mut handle = http::handle();
        let request = handle.get(uri).header("user-agent", "Mozilla/4.0 (compatible)");
        let response = request.exec().unwrap();
        response.move_body()
    }

    fn determine_latest_version() -> Version {
        let body = FetchCommand::execute_request_to_uri("https://api.github.com/repos/slicebuild/slices/branches");
        let body = String::from_utf8(body).unwrap();
        let json = Json::from_str(&body).unwrap();
        let array = json.as_array().unwrap();
        FetchCommand::choose_latest_version(array)
    }

    fn download_latest_version() -> Vec<u8> {
        let version = FetchCommand::determine_latest_version();
        let uri = format!("https://codeload.github.com/slicebuild/slices/zip/{}", version);
        FetchCommand::execute_request_to_uri(&uri)
    }

    fn extract_archive_into_directory(mut zip_archive: ZipArchive<Cursor<Vec<u8>>>, path: PathBuf) {
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

impl Command for FetchCommand {
    fn run(&mut self) {
        let bytes = FetchCommand::download_latest_version();
        let cursor = Cursor::new(bytes);
        let zip_archive = ZipArchive::new(cursor).unwrap();
        FetchCommand::extract_archive_into_directory(zip_archive, Path::new(&self.slices_directory).to_path_buf());
    }
}
