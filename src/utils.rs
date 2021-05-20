use bytes::Bytes;
use dirs::config_dir;
use log::*;
use reqwest::blocking::Response;
use std::path::PathBuf;

pub fn parse_bytes_from_response(response_result: reqwest::Result<Response>) -> Option<Bytes> {
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

pub fn get_config_path() -> PathBuf {
    let mut path = config_dir().expect("No config dir found");
    path.push("QInjector");
    path
}
