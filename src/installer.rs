use crate::configuration::LocalMaps;
use log::*;
use reqwest::blocking::ClientBuilder;
use std::fs::write;
use std::path::Path;

pub struct Installer {
    installed_maps: Vec<String>,
    download_dir: String,
    path_string: String,
}

impl Installer {
    pub fn new() -> Self {
        let local_maps = LocalMaps::new();
        let installed_maps = local_maps
            .maps()
            .iter()
            .map(|map| map.id().to_owned())
            .collect();
        let download_dir = String::from("/home/samuwen/Downloads");
        Self {
            installed_maps,
            download_dir,
            path_string: String::new(),
        }
    }

    pub fn path_string(self, path_string: String) -> Self {
        Self {
            installed_maps: self.installed_maps,
            download_dir: self.download_dir,
            path_string,
        }
    }

    pub fn get_path_string(&self) -> String {
        self.path_string.to_owned()
    }

    pub fn is_map_installed(&self, map_id: &String) -> bool {
        self.installed_maps.iter().any(|map| map == map_id)
    }

    pub fn install_map(&mut self, map_id: String) {
        self.get_file_from_remote(map_id);
    }

    fn get_file_from_remote(&mut self, map_id: String) {
        trace!("Getting file from remote");
        let url = format!("https://www.quaddicted.com/filebase/{}.zip", map_id);
        debug!("Getting file from url: {}", url);
        let client = ClientBuilder::new()
            .timeout(std::time::Duration::from_secs(60))
            .build()
            .unwrap();
        let response_res = client.get(url).send();
        match response_res {
            Ok(response) => {
                let byte_res = response.bytes();
                match byte_res {
                    Ok(bytes) => {
                        let path_str = format!("{}/{}.zip", self.download_dir, map_id);
                        let path = Path::new(&path_str);
                        match write(path, bytes) {
                            Ok(_) => {
                                debug!("Finished writing zip to downloads");
                                self.installed_maps.push(map_id.to_owned());
                            }
                            Err(e) => {
                                error!("Error: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        error!("Error: {}", e);
                    }
                }
            }
            Err(e) => {
                error!("Error: {}", e);
            }
        }
    }
}
