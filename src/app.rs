use crate::configuration::*;
use crate::gui_data::*;
use crate::initialize_gui::*;
use crate::quake_file::*;
use bytes::Bytes;
use getset::{Getters, Setters};
use log::*;
use reqwest::blocking::{ClientBuilder, Response};
use std::cell::RefCell;
use std::fmt::Display;
use std::fs::File;
use std::fs::{remove_file, write};
use std::io::{BufReader, Read};
use std::path::Path;
use std::process::{Command, Output};
use std::rc::Rc;
use zip::read::ZipFile;
use zip::ZipArchive;

#[derive(Clone, Getters, Setters)]
pub struct QInjector {
    #[getset(get = "pub")]
    files: Vec<QuakeFile>,
    #[getset(get = "pub", set = "pub")]
    config: Configuration,
    #[getset(get = "pub")]
    local_maps: LocalMaps,
}

impl QInjector {
    pub fn get_file_by_id(&self, id: String) -> Option<&QuakeFile> {
        trace!("Getting file with id: {}", id);
        self.files.iter().find(|file| file.id() == &id)
    }

    pub fn get_mut_file_by_id(&mut self, id: &String) -> Option<&mut QuakeFile> {
        self.files.iter_mut().find(|file| file.id() == id)
    }

    pub fn set_quake_exe(&mut self, path: String) {
        self.config.set_quake_exe(path);
    }

    pub fn set_quake_dir(&mut self, path: String) {
        self.config.set_quake_dir(path);
    }

    pub fn set_download_dir(&mut self, path: String) {
        self.config.set_download_dir(path);
    }

    pub fn set_rogue_check(&mut self, val: bool) {
        self.config.set_rogue_installed(val);
    }

    pub fn set_hip_check(&mut self, val: bool) {
        self.config.set_hip_installed(val);
    }

    pub fn write_config(&self) {
        self.config.write_to_file();
    }

    /// Initializes map installation
    ///
    /// Gets file from remote server, saves it to a zip file, then retrieves the files contained
    /// within and separates them into files that have duplicate values on the local machine or
    /// not.
    ///
    /// Returns option for two vectors, files that can be installed and files that have duplicate
    /// entries on the file system.
    pub fn start_map_install(&mut self, id: &String) -> Option<(Vec<String>, Vec<String>)> {
        let bytes = match self.get_file_from_remote(id) {
            Some(b) => b,
            None => {
                return None;
            }
        };
        let path_str = format!("{}/{}.zip", self.config.download_dir(), id);
        let path = Path::new(&path_str);
        if !self.write_file_to_disk(bytes, &path) {
            return None;
        }
        self.process_zip_files(path)
    }

    /// Reaches out to the qaddicted server and retrieves a given map's zip file.
    ///
    /// Returns an option for raw bytes.
    fn get_file_from_remote(&self, id: &String) -> Option<bytes::Bytes> {
        trace!("Getting file from remote");
        let url = format!("https://www.quaddicted.com/filebase/{}.zip", id,);
        debug!("Getting file from url: {}", url);
        let client = ClientBuilder::new()
            .timeout(std::time::Duration::from_secs(60))
            .build()
            .unwrap();
        let response_res = client.get(url).send();
        parse_bytes_from_remote(response_res)
    }

    /// Writes a zip file to the local disk.
    ///
    /// Takes in a bytes object and a path, returns a boolean if it wrote successfully.
    fn write_file_to_disk(&self, bytes: bytes::Bytes, path: &Path) -> bool {
        debug!("Writing out to {}", path.to_str().unwrap());
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

    /// Reads through the list of zipped files and determines if any entries are on the file
    /// system.
    ///
    /// Loads a zip file on the file system into memory, then gets a list of files in the
    /// extraction path. Compares the two lists and separates things out into files that we are
    /// clear to install and ones that have duplicate names on the file system. Of note - we check
    /// that the file has a different checksum than the one on the system to install. If the files
    /// are exact dupes of eachother we just remove it from the lists entirely (no work to do).
    fn process_zip_files(&self, zip_path: &Path) -> Option<(Vec<String>, Vec<String>)> {
        debug!("Zip path: {}", zip_path.to_str().unwrap());
        trace!("Unpacking zip archive");
        let reader = BufReader::new(File::open(&zip_path).unwrap());
        let zip_res = ZipArchive::new(reader);
        let mut zip = match zip_res {
            Ok(z) => z,
            Err(e) => {
                error!("Failed to open zip from remote {}", e);
                return None;
            }
        };
        let extract_path = format!("{}/id1/maps", self.config.quake_dir());
        let files_in_dir = match std::fs::read_dir(&extract_path) {
            Ok(files) => {
                trace!("Files in dir: {:?}", files);
                files
            }
            Err(e) => {
                error!(
                    "Failed to read files in the source directory. Did you set things up properly?"
                );
                error!("{}", e);
                return None;
            }
        };
        let entries: Vec<std::fs::DirEntry> = files_in_dir.map(|f| f.unwrap()).collect();
        let mut files_to_extract: Vec<String> = vec![];
        let mut dupe_names: Vec<String> = vec![];
        for i in 0..zip.len() {
            let file_res = zip.by_index(i);
            match file_res {
                Ok(f) => {
                    let zip_file_name = f.name().to_ascii_lowercase();
                    debug!("zip file name: {}", zip_file_name);
                    let found = entries.iter().any(|entry| {
                        let names_match = entry.file_name() == zip_file_name.as_str();
                        trace!(
                            "Checking for dupe file names: {:?} == {}",
                            entry.file_name(),
                            zip_file_name
                        );
                        let mut crc_match = false;
                        // if the names match, do the more intensive crc check
                        if names_match {
                            let file = File::open(entry.path()).unwrap();
                            let mut bytes: Vec<u8> = file.bytes().map(|b| b.unwrap()).collect();
                            let checksum = crc::crc32::checksum_ieee(&mut bytes);
                            trace!(
                                "Checking for dupe file crc: {} != local file crc: {}",
                                f.crc32(),
                                checksum
                            );
                            // validate that names match but crcs do not
                            crc_match = checksum != f.crc32();
                        }
                        trace!("{} && {}", names_match, crc_match);
                        return names_match && crc_match;
                    });
                    debug!("Found: {}", found);

                    match found {
                        true => {
                            debug!("Found duplicate file in unpacking");
                            dupe_names.push(zip_file_name.to_owned());
                        }
                        false => files_to_extract.push(zip_file_name.to_owned()),
                    }
                }
                Err(e) => {
                    error!("Failed to extract file at index: {}\n{}", i, e);
                    return None;
                }
            };
        }
        Some((files_to_extract, dupe_names))
    }

    pub fn extract_data_from_zip(&mut self, to_install: &Vec<String>, id: &String) -> bool {
        trace!("Extracting data from zip");
        let mut files = vec![];
        let extract_path = format!("{}/id1/maps", self.config.quake_dir());
        let path_str = format!("{}/{}.zip", self.config.download_dir(), id);
        let path = Path::new(&path_str);
        let mut archive = self.get_zip_archive(path).unwrap();
        info!("Starting zip extraction");
        let all_extracted = to_install.iter().all(|file_name| {
            debug!("Extracting: {}", file_name);
            let position = archive
                .file_names()
                .position(|name| &name.to_ascii_lowercase() == file_name)
                .unwrap();
            let mut file = archive
                .by_index(position)
                .expect("File was not found in zip archive");
            let info = FileInfoBuilder::default()
                .crc(file.crc32())
                .name(file.name().to_ascii_lowercase())
                .build()
                .unwrap();
            files.push(info);
            self.extract_file_to_directory(&mut file, &extract_path)
        });
        match all_extracted {
            true => {
                let map_pack = MapPackBuilder::default()
                    .id(id.to_owned())
                    .files(files)
                    .build();
                match map_pack {
                    Ok(pack) => {
                        self.local_maps.add_map(pack);
                        debug!("Added {} to local maps", id);
                        true
                    }
                    Err(e) => {
                        error!("Couldn't build map pack: {}", e);
                        false
                    }
                }
            }
            false => false,
        }
    }

    pub fn uninstall_map(&mut self, id: &String) -> bool {
        trace!("Uninstalling current selected map");
        let map = match self.local_maps.get_map_by_id(&id) {
            Some(m) => m,
            None => {
                warn!("Couldn't find {} map to uninstall", &id);
                return false;
            }
        };
        map.files().iter().for_each(|file| {
            let file_path = format!("{}/id1/maps/{}", self.config.quake_dir(), file.name());
            match remove_file(&file_path) {
                Ok(_) => debug!("Removed file at: {}", file_path),
                Err(e) => {
                    warn!("Couldn't remove file at: {}", file_path);
                    warn!("Error: {}", e);
                }
            }
        });
        self.local_maps.remove_map(id.to_owned());
        true
    }

    pub fn update_current_file_download_status(&mut self, status: bool, id: &String) {
        match self.get_mut_file_by_id(id) {
            Some(file) => {
                file.set_is_local(status);
            }
            None => {
                error!("Attempting to get file by invalid id: {}", id);
            }
        }
    }

    pub fn play_quake_map(&self, id: &String, start_map: &String) -> std::io::Result<Output> {
        let exe = self.config.quake_exe();
        let dir = self.config.quake_dir();
        info!("Attempting to play game: {}", id);
        debug!("start map: {}", start_map);
        let map = match self.get_file_by_id(id.to_owned()) {
            Some(m) => m,
            None => {
                warn!("Couldn't find {} map to play", &id);
                panic!("yikes!");
            }
        };
        let mut cmd = Command::new(exe);
        cmd.arg("-basedir").arg(dir);
        if let Some(line) = map.tech_info().command_line() {
            debug!("Adding command line: {}", line);
            line.split(" ").for_each(|sp| {
                cmd.arg(sp);
            });
        }
        cmd.arg("+map").arg(start_map);
        cmd.output()
    }

    fn get_zip_archive(&self, zip_path: &Path) -> Option<ZipArchive<BufReader<File>>> {
        trace!("Unpacking zip archive");
        let reader = BufReader::new(File::open(zip_path).unwrap());
        let zip_res = ZipArchive::new(reader);
        match zip_res {
            Ok(z) => Some(z),
            Err(e) => {
                error!("Failed to open zip from remote {}", e);
                None
            }
        }
    }

    pub fn write_local_maps(&self) {
        self.local_maps.write_to_file();
    }

    fn extract_file_to_directory<P: AsRef<Path> + Display>(
        &self,
        f: &mut ZipFile,
        extract_path: P,
    ) -> bool {
        let name = f.name().to_ascii_lowercase();
        debug!("Extracting {}", name);
        let f_path = format!("{}/{}", extract_path, name);
        let mut zip_file = match File::create(&f_path) {
            Ok(z_file) => {
                trace!("Successfully extracted {} to {}", name, f_path);
                z_file
            }
            Err(e) => {
                error!("Failed to extract {}: {}", name, e);
                return false;
            }
        };
        std::io::copy(f, &mut zip_file).expect("Couldn't copy");
        true
    }

    pub fn load_map_image(&self, id: String) -> gdk_pixbuf::Pixbuf {
        let path = format!("images/{}.jpg", id);
        let path = Path::new(&path);
        debug!("Attempting to load image at: {:?}", path);
        match gdk_pixbuf::Pixbuf::from_file_at_size(path, 200, 200) {
            Ok(pixbuf) => pixbuf,
            Err(_) => {
                // gotta get from remote
                trace!("Getting image file from remote");
                let url = format!("https://www.quaddicted.com/reviews/screenshots/{}.jpg", id);
                let client = ClientBuilder::new()
                    .timeout(std::time::Duration::from_secs(60))
                    .build()
                    .unwrap();
                let response_res = client.get(url).send();
                let byte_opt = parse_bytes_from_remote(response_res);
                if let Some(bytes) = byte_opt {
                    self.write_file_to_disk(bytes, &path);
                }
                gdk_pixbuf::Pixbuf::from_file_at_size(path, 200, 200).unwrap()
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

pub fn initialize_application() {
    trace!("Starting application");
    // make sure image directory exists
    match std::fs::create_dir("images") {
        Ok(_) => {
            trace!("Made images directory");
        }
        Err(_) => {
            trace!("Image directory already exists");
        }
    }
    let (sender, receiver): (glib::Sender<String>, glib::Receiver<String>) =
        glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    let config = Configuration::new();
    let local_maps = LocalMaps::new();
    let data = initialize_data(&local_maps);
    let inj = QInjector {
        files: data,
        config,
        local_maps,
    };
    let inj = Rc::new(RefCell::new(inj));
    let gui_data = GuiData::new();
    initialize_gui(&gui_data, inj);
}
