use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use super::PuzzleInfo;

pub fn get_data(pi: &PuzzleInfo, session: &str) -> Result<String, GetDataError> {
    read_local_data().or_else(|_| {
        create_resource_dir()
            .and_then(|_| fetch_data(&pi, session))
            .and_then(|data| {
                save_data(data.as_str()).and(Ok(data))
            })
    })
}

fn local_data_path() -> PathBuf {
    Path::new(".").join("resources").join("data")
}

fn read_local_data() -> Result<String, io::Error> {
    fs::read_to_string(local_data_path())
}

fn create_resource_dir() -> Result<(), GetDataError> {
    match fs::create_dir(local_data_path().parent().unwrap()) {
        Ok(()) => Ok(()),
        Err(error) => match error.kind() {
            io::ErrorKind::AlreadyExists => Ok(()),
            _ => Err(GetDataError::IOError(error))
        }
    }
}

fn fetch_data(pi: &PuzzleInfo, session: &str) -> Result<String, GetDataError> {
    let endpoint = format!("https://adventofcode.com/{}/day/{}/input", pi.year, pi.day);
    let client = reqwest::blocking::Client::new();
    client
        .request(reqwest::Method::GET, endpoint)
        .header("cookie", format!("session={}", session))
        .send()
        .and_then(|resp| resp.text())
        .map_err(|err| GetDataError::HttpError(err))
}

fn save_data(data: &str) -> Result<(), GetDataError> {
    fs::write(local_data_path(), data)
        .map_err(|err| GetDataError::IOError(err))
}

#[derive(thiserror::Error, Debug)]
pub enum GetDataError {
    #[error("lol")]
    IOError(#[from] io::Error),
    #[error("lol2")]
    HttpError(#[from] reqwest::Error),
}
