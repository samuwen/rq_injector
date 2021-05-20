use crate::utils::*;
use bytes::Bytes;
use getset::Getters;
use log::*;
use reqwest::blocking::ClientBuilder;
use std::fs::{write, File};
use std::path::PathBuf;

#[derive(Clone, Debug, Default, Getters)]
#[getset(get = "pub")]
pub struct ImageLoader {
    path: PathBuf,
    map_id: String,
    path_string: String,
}

impl ImageLoader {
    pub fn new(map_id: String, path_string: String) -> Self {
        Self {
            path: PathBuf::new(),
            path_string,
            map_id,
        }
    }

    pub fn load_map_image(&mut self) {
        let mut path = get_config_path();
        path.push("images");
        path.push(format!("{}.jpg", self.map_id));
        self.path = path;
        debug!("Attempting to load image at: {:?}", self.path);
        let file_result = File::open(&self.path);
        if let Err(_) = file_result {
            // gotta get from remote
            trace!("Getting image file from remote");
            let url = format!(
                "https://www.quaddicted.com/reviews/screenshots/{}.jpg",
                self.map_id
            );
            let client = ClientBuilder::new()
                .timeout(std::time::Duration::from_secs(60))
                .build()
                .unwrap();
            let response_res = client.get(url).send();
            let byte_opt = parse_bytes_from_response(response_res);
            if let Some(bytes) = byte_opt {
                debug!("Writing out to path: {:?}", self.path);
                write_file_to_disk(&bytes, &self.path);
            } else {
                panic!("Failed to get dem bytes");
            }
        }
    }
}

fn write_file_to_disk(bytes: &Bytes, path: impl AsRef<std::path::Path>) -> bool {
    match write(path, bytes) {
        Ok(_) => info!("Wrote to file successfully"),
        Err(e) => {
            error!("Couldn't write to file: {}", e);
            println!("Unable to write file out. Do you have permissions?");
            return false;
        }
    }
    true
}
