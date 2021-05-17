use getset::Getters;
use log::*;
use quick_xml::de::{from_reader, DeError};
use serde::Deserialize;
use std::fs::File;
use std::io::{BufReader, Read};

pub fn initialize_data() -> Vec<QuakeFile> {
    let file = File::open("data.xml").expect("file not found");
    let reader = BufReader::new(file);
    read_datastore(reader).files().clone()
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

#[derive(Clone, Debug, Deserialize, Getters)]
#[getset(get = "pub")]
pub struct Files {
    #[serde(rename = "file", default)]
    files: Vec<QuakeFile>,
}

#[derive(Clone, Debug, Deserialize, Getters)]
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

impl QuakeFile {
    pub fn set_is_local(&mut self, val: bool) {
        self.installed_locally = val;
    }
}

#[derive(Clone, Debug, Deserialize, Getters)]
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

#[derive(Clone, Debug, Deserialize, Getters)]
#[getset(get = "pub")]
pub struct Requirements {
    #[serde(rename = "file")]
    req_file: Vec<ReqFile>,
}

#[derive(Clone, Debug, Deserialize, Getters)]
#[getset(get = "pub")]
pub struct ReqFile {
    id: String,
}
