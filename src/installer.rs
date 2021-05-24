use crate::configuration::*;
use crate::request_utils::get_map_from_remote;
use getset::Getters;
use log::*;
use std::fs::{create_dir, create_dir_all, read_dir, remove_file, File};
use std::io::{BufReader, Write};
use zip::ZipArchive;

#[derive(Clone, Getters)]
#[getset(get = "pub")]
pub struct Installer {
    download_dir: String,
    quake_dir: String,
    path_string: String,
    map_id: String,
    installed_map_pack: Option<MapPack>,
}

impl Installer {
    pub fn new() -> Self {
        Self {
            download_dir: String::new(),
            path_string: String::new(),
            quake_dir: String::new(),
            installed_map_pack: None,
            map_id: String::new(),
        }
    }

    pub fn with_path_string(self, string: String) -> Self {
        Self {
            download_dir: self.download_dir,
            path_string: string,
            quake_dir: self.quake_dir,
            installed_map_pack: self.installed_map_pack,
            map_id: self.map_id,
        }
    }

    pub fn with_download_dir(self, dir: String) -> Self {
        Self {
            download_dir: dir,
            path_string: self.path_string,
            quake_dir: self.quake_dir,
            installed_map_pack: self.installed_map_pack,
            map_id: self.map_id,
        }
    }

    pub fn with_quake_dir(self, dir: String) -> Self {
        Self {
            download_dir: self.download_dir,
            path_string: self.path_string,
            quake_dir: dir,
            installed_map_pack: self.installed_map_pack,
            map_id: self.map_id,
        }
    }

    pub fn with_map_id(self, id: String) -> Self {
        Self {
            download_dir: self.download_dir,
            path_string: self.path_string,
            quake_dir: self.quake_dir,
            installed_map_pack: self.installed_map_pack,
            map_id: id,
        }
    }

    pub fn install_map(&mut self, map_id: String) {
        trace!("Started installing: {}", map_id);
        if !self.is_map_zip_downloaded(&map_id) {
            debug!("Zip not found. Grabbing from remote");
            get_map_from_remote(&map_id, &self.download_dir);
        } else {
            debug!("Local file found. Stop all the downloading");
        }
        self.unpack_zip_to_dir(&map_id);
        trace!("Done installing: {}", map_id);
    }

    pub fn uninstall_map(&mut self, files: Vec<FileInfo>) {
        trace!("Starting uninstalling: {}", self.map_id);
        files.iter().for_each(|file| {
            let file_path = format!("{}/id1/maps/{}", self.quake_dir, file.name());
            match remove_file(&file_path) {
                Ok(_) => debug!("Removed file at: {}", file_path),
                Err(e) => {
                    warn!("Couldn't remove file at: {}", file_path);
                    warn!("Error: {}", e);
                }
            }
        });
        trace!("Done uninstalling: {}", self.map_id);
    }

    fn is_map_zip_downloaded(&self, map_id: &String) -> bool {
        info!("Checking download dir for file: {:?}", self.download_dir);
        let mut entries = read_dir(self.download_dir.to_string()).expect("reading dir failed");
        entries.any(|entry| {
            let e = match entry {
                Ok(e) => e,
                Err(e) => panic!("{}", e),
            };
            let zip_name = format!("{}.zip", map_id);
            e.file_name() == std::ffi::OsString::from(zip_name)
        })
    }

    fn unpack_zip_to_dir(&mut self, map_id: &String) {
        let mut archive = self.get_zip_archive(map_id);
        let has_extra_dirs = archive.file_names().any(|name| name.contains("/"));
        match has_extra_dirs {
            true => {
                debug!("We have extra directories, lets create those");
                let root_dir_name = format!("{}/{}", self.quake_dir, &map_id);
                create_dir(&root_dir_name).expect("Quake dir probably not set");
                match archive.extract(&root_dir_name) {
                    Ok(_) => debug!("Extraction went well"),
                    Err(e) => error!("Failed to extract zip: {}", e),
                };
            }
            false => {
                debug!("No extra directories, creating extra dir structure");
                let root_dir_name = format!("{}/{}", self.quake_dir, &map_id);
                let map_dir_name = format!("{}/maps", &root_dir_name);
                create_dir_all(&map_dir_name).expect("Quake dir probably not set");
                let mut bsp_name = String::from("start");
                for i in 0..archive.len() {
                    let mut file = archive.by_index(i).unwrap();
                    let name = file.name().to_ascii_lowercase();
                    debug!("Extracting: {}", name);
                    let file_path = match name.contains("bsp") {
                        true => {
                            // get the root file name minus .bsp
                            bsp_name = name.split(".").next().unwrap().to_string();
                            format!("{}/{}", &map_dir_name, &name)
                        }
                        false => format!("{}/{}", &root_dir_name, &name),
                    };
                    debug!("Writing out to local file: {}", file_path);
                    let mut local_file = File::create(&file_path).unwrap();
                    std::io::copy(&mut file, &mut local_file)
                        .expect("Couldn't copy zip file to disk");
                    trace!("Successfully wrote {} to disk", name);
                }
                let auto_file_path = format!("{}/autoexec.cfg", &root_dir_name);
                let mut f = File::create(&auto_file_path).expect("Couldn't create autoexec file");
                write!(f, "map {}", bsp_name).expect("Couldn't write auto file");
            }
        }
        let map_pack = MapPackBuilder::default()
            .id(map_id.to_owned())
            .files(Vec::new())
            .build()
            .unwrap();
        self.installed_map_pack = Some(map_pack);
    }

    fn get_zip_archive(&self, map_id: &String) -> ZipArchive<BufReader<File>> {
        let path = format!("{}/{}.zip", self.download_dir, map_id);
        debug!("Opening zip archive: {}", path);
        let reader = BufReader::new(File::open(path).unwrap());
        ZipArchive::new(reader).unwrap()
    }
}
