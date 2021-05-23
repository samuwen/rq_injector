use crate::locales::{init_locale, Locale, LocaleChoice, Nester};
use derive_builder::Builder;
use dirs::config_dir;
use getset::{Getters, Setters};
use log::*;
use quick_xml::de::from_reader;
use quick_xml::se::to_writer;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::{Path, PathBuf};

const CONFIG_FILE_NAME: &str = "config.xml";
const LOCAL_MAPS_FILE_NAME: &str = "installedMaps.xml";

#[derive(Clone, Debug, Deserialize, Getters, Serialize, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct Configuration {
    #[serde(default = "Default::default")]
    quake_dir: String,
    #[serde(default = "Default::default")]
    quake_exe: String,
    #[serde(default = "Default::default")]
    download_dir: String,
    rogue_installed: bool,
    hip_installed: bool,
    is_offline: bool,
    config_dir: PathBuf,
    image_cache_dir: PathBuf,
    image_resources_dir: PathBuf,
    current_locale_choice: Nester,
    #[serde(skip)] // don't store this in the file. Makes the file hard to read.
    current_locale: Locale,
}

impl Configuration {
    pub fn new() -> Self {
        let mut file_path = config_dir().expect("No config dir found");
        file_path.push("QInjector");
        file_path.push(CONFIG_FILE_NAME);
        let mut config: Self = read_or_initialize(file_path, "config");
        trace!("Config object: {:?}", config);
        let mut locale_resources_dir = std::env::current_dir().expect("No current dir found");
        locale_resources_dir.push("resources");
        locale_resources_dir.push("locales");
        let locale = init_locale(
            locale_resources_dir.clone(),
            config.current_locale_choice.get_choice(),
        );
        trace!("Locale object: {:?}", locale);
        config.current_locale = locale;
        config
    }

    pub fn write_to_file(&self) {
        let mut file_path = self.config_dir.clone();
        file_path.push(CONFIG_FILE_NAME);
        write_to_file(file_path, self, "config");
    }
}

impl Default for Configuration {
    fn default() -> Self {
        let mut config_dir = config_dir().expect("No config dir found");
        config_dir.push("QInjector");
        let mut image_cache_dir = config_dir.clone();
        image_cache_dir.push("images");
        let mut image_resources_dir = std::env::current_dir().expect("No current dir found");
        image_resources_dir.push("resources");
        image_resources_dir.push("images");
        let mut locale_resources_dir = std::env::current_dir().expect("No current dir found");
        locale_resources_dir.push("resources");
        locale_resources_dir.push("locales");
        let locale = init_locale(locale_resources_dir, LocaleChoice::EnUs.get_name());
        // real tired of selecting these through the GUI haha
        let quake_dir = match cfg!(debug_assertions) {
            true => "/home/samuwen/.steam/debian-installation/steamapps/common/Quake".to_string(),
            false => Default::default(),
        };
        let quake_exe = match cfg!(debug_assertions) {
            true => "/usr/games/quakespasm".to_string(),
            false => Default::default(),
        };
        let download_dir = match cfg!(debug_assertions) {
            true => "/home/samuwen/Downloads".to_string(),
            false => Default::default(),
        };
        Self {
            quake_dir,
            quake_exe,
            download_dir,
            rogue_installed: Default::default(),
            hip_installed: Default::default(),
            is_offline: Default::default(),
            config_dir,
            image_cache_dir,
            image_resources_dir,
            current_locale_choice: Nester::NestedEnum(LocaleChoice::EnUs),
            current_locale: locale,
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Getters, Serialize, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct LocalMaps {
    #[serde(rename = "map", default)]
    maps: Vec<MapPack>,
}

impl LocalMaps {
    pub fn new(config_dir: PathBuf) -> Self {
        let mut file_path = config_dir;
        file_path.push(LOCAL_MAPS_FILE_NAME);
        debug!("Local file path: {:?}", file_path);
        let maps = read_or_initialize(file_path, "local maps");
        trace!("{:?}", maps);
        maps
    }

    pub fn add_map(&mut self, pack: MapPack) {
        self.maps.push(pack);
    }

    pub fn remove_map(&mut self, id: &String) {
        let map_pos = match self.maps.iter().position(|map| &map.id == id) {
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

    pub fn is_map_installed(&self, id: &String) -> bool {
        self.maps.iter().any(|map| map.id() == id)
    }

    pub fn write_to_file(&self, config_dir: PathBuf) {
        let mut file_path = config_dir;
        file_path.push(LOCAL_MAPS_FILE_NAME);
        write_to_file(file_path, self, "local maps");
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

fn read_or_initialize<T: DeserializeOwned + Default>(path: impl AsRef<Path>, name: &str) -> T {
    trace!("Initializing {}", name);
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
        Err(e) => {
            info!("{} file not found. Generating default", name);
            trace!("Error message: {}", e);
            T::default()
        }
    }
}

fn write_to_file<T: Serialize>(path: impl AsRef<Path>, obj: &T, name: &str) {
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
