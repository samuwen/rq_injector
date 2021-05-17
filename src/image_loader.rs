use bytes::Bytes;
use getset::Getters;
use log::*;
use reqwest::blocking::{ClientBuilder, Response};
use std::fs::{write, File};

#[derive(Clone, Debug, Default, Getters)]
#[getset(get = "pub")]
pub struct ImageLoader {
    path: String,
    map_id: String,
    path_string: String,
}

impl ImageLoader {
    pub fn new(map_id: String, path_string: String) -> Self {
        Self {
            path: String::new(),
            path_string,
            map_id,
        }
    }

    pub fn load_map_image(&mut self) {
        let path = format!("images/{}.jpg", self.map_id);
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
            let byte_opt = parse_bytes_from_remote(response_res);
            if let Some(bytes) = byte_opt {
                write_file_to_disk(&bytes, &self.path);
            } else {
                panic!("Failed to get dem bytes");
            }
        }
    }
}

fn parse_bytes_from_remote(response_result: reqwest::Result<Response>) -> Option<Bytes> {
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

fn write_file_to_disk(bytes: &Bytes, path: &String) -> bool {
    debug!("Writing out to {}", path);
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
