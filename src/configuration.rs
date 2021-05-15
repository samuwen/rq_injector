use derive_builder::Builder;
use getset::{Getters, Setters};
use log::*;
use quick_xml::de::from_reader;
use quick_xml::se::to_writer;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, Write};

const CONFIG_FILE_PATH: &str = "config.xml";
const LOCAL_MAPS_FILE_PATH: &str = "installedMaps.xml";

#[derive(Clone, Default, Debug, Deserialize, Getters, Serialize, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct Configuration {
    quake_dir: String,
    quake_exe: String,
    download_dir: String,
    rogue_installed: bool,
    hip_installed: bool,
}

impl Configuration {
    pub fn new() -> Self {
        let config = read_or_initialize(CONFIG_FILE_PATH, "config");
        trace!("{:?}", config);
        config
    }

    pub fn write_to_file(&self) {
        write_to_file(CONFIG_FILE_PATH, self, "config");
    }
}

#[derive(Clone, Debug, Default, Deserialize, Getters, Serialize, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct LocalMaps {
    #[serde(rename = "map", default)]
    maps: Vec<MapPack>,
}

impl LocalMaps {
    pub fn new() -> Self {
        let maps = read_or_initialize(LOCAL_MAPS_FILE_PATH, "local maps");
        trace!("{:?}", maps);
        maps
    }

    pub fn add_map(&mut self, pack: MapPack) {
        self.maps.push(pack);
    }

    pub fn remove_map(&mut self, id: String) {
        let map_pos = match self.maps.iter().position(|map| map.id == id) {
            Some(m) => m,
            None => {
                error!("Couldn't find map '{}' to remove from local list", id);
                return;
            }
        };
        self.maps.remove(map_pos);
    }

    pub fn get_map_by_id(&self, id: &String) -> Option<&MapPack> {
        self.maps.iter().find(|map| map.id() == id)
    }

    pub fn write_to_file(&self) {
        write_to_file(LOCAL_MAPS_FILE_PATH, self, "local maps");
    }
}

#[derive(Builder, Clone, Debug, Default, Deserialize, Getters, Setters, Serialize)]
#[getset(get = "pub", set = "pub")]
pub struct MapPack {
    id: String,
    #[serde(rename = "file", default)]
    files: Vec<FileInfo>,
}

#[derive(Builder, Clone, Debug, Default, Deserialize, Getters, Setters, Serialize)]
#[getset(get = "pub", set = "pub")]
pub struct FileInfo {
    crc: u32,
    name: String,
}

fn read_or_initialize<T: DeserializeOwned + Default>(path: &str, name: &str) -> T {
    trace!("Initializing {} to path: {}", name, path);
    let file_opt = File::open(path);
    match file_opt {
        Ok(f) => {
            info!("{} file found. Starting parse", name);
            let reader = BufReader::new(f);
            let deserialized_result = from_reader(reader);
            match deserialized_result {
                Ok(c) => {
                    info!("{} file parsed successfully", name);
                    c
                }
                Err(e) => {
                    warn!("Error reading {} file: {}", name, e);
                    warn!("Generating empty default");
                    T::default()
                }
            }
        }
        Err(_) => {
            info!("{} file not found. Generating default", name);
            T::default()
        }
    }
}

fn write_to_file<T: Serialize>(path: &str, obj: &T, name: &str) {
    trace!("Writing {} to file", name);
    let mut file = File::create(path).unwrap();
    match to_writer(&mut file, obj) {
        Ok(_) => (),
        Err(e) => {
            error!("Failed to serialize {}: {}", name, e);
            return;
        }
    };
    file.flush()
        .expect("unable to write file for some dumb reason");
}
