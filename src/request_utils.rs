use bytes::Bytes;
use log::*;
use reqwest::blocking::{ClientBuilder, Response};
use std::fmt::Debug;
use std::fs::{write, File};
use std::path::{Path, PathBuf};

pub fn get_database_from_remote(file_path: PathBuf) -> File {
    debug!("Getting database from remote");
    let url = "https://www.quaddicted.com/reviews/quaddicted_database.xml".to_string();
    if let Some(bytes) = handle_basic_request(url) {
        write_file(&file_path, bytes);
        return File::open(file_path).unwrap();
    };
    // TODO: Fix
    panic!("Couldn't parse database!!")
}

pub fn get_map_from_remote(map_id: &String, download_dir: &String) {
    trace!("Getting map from remote");
    let url = format!("https://www.quaddicted.com/filebase/{}.zip", map_id);
    debug!("Getting map from url: {}", url);
    let path = format!("{}/{}.zip", download_dir, map_id);
    if let Some(bytes) = handle_basic_request(url) {
        write_file(&path, bytes);
    }
}

pub fn get_image_from_remote<P: AsRef<Path> + Debug>(map_id: &String, path: P) {
    trace!("Getting image file from remote");
    let url = format!(
        "https://www.quaddicted.com/reviews/screenshots/{}.jpg",
        map_id
    );
    debug!("Getting image from url: {}", url);
    if let Some(bytes) = handle_basic_request(url) {
        write_file(&path, bytes);
    }
}

fn write_file<P: AsRef<Path> + Debug>(path: P, bytes: Bytes) {
    match write(&path, bytes) {
        Ok(_) => {
            debug!("Wrote file to {:?}", path);
        }
        Err(e) => {
            error!("Error: {}", e);
        }
    };
}

fn handle_basic_request(url: String) -> Option<Bytes> {
    let client = ClientBuilder::new()
        .timeout(std::time::Duration::from_secs(60))
        .build()
        .unwrap();
    let response_res = client.get(url).send();
    parse_bytes_from_response(response_res)
}

fn parse_bytes_from_response(response_result: reqwest::Result<Response>) -> Option<Bytes> {
    match response_result {
        Ok(res) => match res.bytes() {
            Ok(b) => {
                debug!("Got file bytes successfully");
                Some(b)
            }
            Err(e) => {
                error!("Couldn't parse file bytes: {}", e);
                println!("File on server is invalid. Try another file");
                None
            }
        },
        Err(e) => {
            error!("Couldn't get data from remote: {}", e);
            println!("Couldn't talk to remote server. Are you connected to the internet?");
            None
        }
    }
}
