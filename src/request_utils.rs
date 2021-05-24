use log::*;
use reqwest::blocking::*;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader, Cursor, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

pub fn get_database_from_remote(file_path: PathBuf) -> File {
    debug!("Getting database from remote");
    let url = "https://www.quaddicted.com/reviews/quaddicted_database.xml".to_string();
    debug!("Database file path: {:?}", file_path);
    get_remote_file_and_write_to_path(url, &file_path);
    return File::open(&file_path).unwrap();
}

pub fn get_map_from_remote(map_id: &String, download_dir: &String) {
    trace!("Getting map from remote");
    let url = format!("https://www.quaddicted.com/filebase/{}.zip", map_id);
    debug!("Getting map from url: {}", url);
    let path = format!("{}/{}.zip", download_dir, map_id);
    get_remote_file_and_write_to_path(url, &path);
}

pub fn get_image_from_remote<P: AsRef<Path> + Debug>(map_id: &String, path: P) {
    trace!("Getting image file from remote");
    let url = format!(
        "https://www.quaddicted.com/reviews/screenshots/{}.jpg",
        map_id
    );
    debug!("Getting image from url: {}", url);
    get_remote_file_and_write_to_path(url, &path);
}

fn get_remote_file_and_write_to_path<P: AsRef<Path> + Debug>(
    url: String,
    path: P,
) -> Result<(), reqwest::Error> {
    let mut response = get(&url)?;
    let content_length = response.content_length().unwrap();

    let mut out_bytes = vec![0; content_length as usize];
    let mut in_bytes = vec![0; 0x4000];
    let mut total = 0;
    'byte_reader: loop {
        match response.read(&mut in_bytes) {
            Ok(b) => {
                if total != 0 {
                    debug!(
                        "progress: {:.2}%",
                        (total as f64 / content_length as f64) * 100.0
                    );
                } else {
                    debug!("progress: 0%");
                }
                out_bytes
                    .write_all(&in_bytes.as_slice()[0..b])
                    .expect("fail");
                total += b;
                if total == content_length as usize {
                    break 'byte_reader;
                }
            }
            Err(e) => {
                error!("Failed to stream data: {}", e);
            }
        }
    }

    let mut file = File::create(&path).expect("Couldn't create file");
    file.write(&out_bytes)
        .expect("Couldn't write chunk for some reason");
    Ok(())
}
