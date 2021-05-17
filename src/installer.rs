use crate::configuration::*;
use bytes::Bytes;
use getset::Getters;
use log::*;
use reqwest::blocking::{ClientBuilder, Response};
use std::fs::{remove_file, write, File};
use std::io::BufReader;
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
    // hack - remove this first init garbage and
    pub fn new(_first_init: bool) -> Self {
        Self {
            download_dir: String::from("/home/samuwen/Downloads"),
            path_string: String::new(),
            quake_dir: String::from(
                "/home/samuwen/.steam/debian-installation/steamapps/common/Quake/id1/maps",
            ),
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
        self.get_file_from_remote(&map_id);
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

    fn get_file_from_remote(&mut self, map_id: &String) {
        trace!("Getting file from remote");
        let url = format!("https://www.quaddicted.com/filebase/{}.zip", map_id);
        debug!("Getting file from url: {}", url);
        let client = ClientBuilder::new()
            .timeout(std::time::Duration::from_secs(60))
            .build()
            .unwrap();
        let response_res = client.get(url).send();
        let bytes_opt = self.parse_bytes_from_response(response_res);
        let bytes = match bytes_opt {
            Some(b) => b,
            None => {
                return;
            }
        };
        let path = format!("{}/{}.zip", self.download_dir, map_id);
        match write(&path, bytes) {
            Ok(_) => {
                debug!("Finished writing zip to downloads");
            }
            Err(e) => {
                error!("Error: {}", e);
            }
        };
    }

    fn unpack_zip_to_dir(&mut self, map_id: &String) {
        let mut archive = self.get_zip_archive(map_id);
        let mut files = vec![];
        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            let name = file.name().to_ascii_lowercase();
            debug!("Extracting: {}", name);
            files.push(
                FileInfoBuilder::default()
                    .crc(file.crc32())
                    .name(name.to_owned())
                    .build()
                    .unwrap(),
            );
            let file_path = format!("{}/{}", self.quake_dir, name);
            debug!("Writing out to local file: {}", file_path);
            let mut local_file = File::create(&file_path).unwrap();
            std::io::copy(&mut file, &mut local_file).expect("Couldn't copy zip file to disk");
            trace!("Successfully wrote {} to disk", name);
        }
        let map_pack = MapPackBuilder::default()
            .id(map_id.to_owned())
            .files(files)
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

    fn parse_bytes_from_response(
        &self,
        response_result: reqwest::Result<Response>,
    ) -> Option<Bytes> {
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
}
