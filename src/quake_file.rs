use bytes::Bytes;
use dirs::config_dir;
use getset::Getters;
use log::*;
use quick_xml::de::{from_reader, DeError};
use reqwest::blocking::{ClientBuilder, Response};
use serde::Deserialize;
use std::fs::write;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

pub fn initialize_data() -> Files {
    let mut file_path = config_dir().expect("Config dir not found");
    file_path.push("QInjector");
    file_path.push("database.xml");
    let file = match File::open(&file_path) {
        Ok(f) => f,
        Err(_) => get_data_from_remote(file_path),
    };
    let reader = BufReader::new(file);
    read_datastore(reader)
}

fn get_data_from_remote(file_path: PathBuf) -> File {
    debug!("Getting database from remote");
    let url = "https://www.quaddicted.com/reviews/quaddicted_database.xml".to_string();
    let client = ClientBuilder::new()
        .timeout(std::time::Duration::from_secs(60))
        .build()
        .unwrap();
    let response_res = client.get(url).send();
    let bytes_opt = parse_bytes_from_response(response_res);
    let bytes = match bytes_opt {
        Some(b) => b,
        None => {
            panic!("Failed to read the remote database");
        }
    };
    match write(&file_path, bytes) {
        Ok(_) => {
            debug!("Finished writing zip to downloads");
        }
        Err(e) => {
            error!("Error: {}", e);
        }
    };
    File::open(file_path).unwrap()
}

fn parse_bytes_from_response(response_result: reqwest::Result<Response>) -> Option<Bytes> {
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

fn read_datastore<R: Read>(reader: BufReader<R>) -> Files {
    trace!("Reading the data file");
    let file_result: Result<Files, DeError> = from_reader(reader);
    let files: Files = match file_result {
        Ok(f) => {
            info!("Data file has parsed successfully");
            f
        }
        Err(e) => {
            error!("Couldn't parse data file: {}", e);
            println!("Your data is corrupt. Please delete the 'database.xml' file and try again");
            std::process::exit(1)
        }
    };
    files
}

#[derive(Clone, Debug, Default, Deserialize, Getters)]
#[getset(get = "pub")]
pub struct Files {
    #[serde(rename = "file", default)]
    files: Vec<QuakeFile>,
}

#[derive(Clone, Debug, Default, Deserialize, Getters)]
#[serde(rename(deserialize = "file"))]
#[getset(get = "pub")]
pub struct QuakeFile {
    id: String,
    #[serde(rename = "type")]
    kind: u8,
    rating: String,
    author: String,
    title: String,
    #[serde(rename = "md5sum")]
    md5: String,
    size: String,
    date: String,
    description: String,
    #[serde(rename = "techinfo")]
    tech_info: TechInfo,
    #[serde(skip)]
    installed_locally: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Getters)]
#[getset(get = "pub")]
pub struct TechInfo {
    #[serde(default, rename = "zipbasedir")]
    zip_base_dir: Option<String>,
    #[serde(default, rename = "commandline")]
    command_line: Option<String>,
    #[serde(default, rename = "startmap")]
    start_map: Vec<String>,
    #[serde(default)]
    requirements: Option<Requirements>,
}

#[derive(Clone, Debug, Default, Deserialize, Getters)]
#[getset(get = "pub")]
pub struct Requirements {
    #[serde(rename = "file")]
    req_file: Vec<ReqFile>,
}

#[derive(Clone, Debug, Default, Deserialize, Getters)]
#[getset(get = "pub")]
pub struct ReqFile {
    id: String,
}
