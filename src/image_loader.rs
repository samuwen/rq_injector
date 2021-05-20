use crate::request_utils::get_image_from_remote;
use crate::utils::*;
use getset::Getters;
use log::*;
use std::fs::File;
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
            get_image_from_remote(&self.map_id, &self.path);
        }
    }
}
