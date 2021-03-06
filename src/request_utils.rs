use crate::download_progress::DownloadProgress;
use glib::Sender;
use log::*;
use reqwest::blocking::*;
use std::fmt::Debug;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::Path;

pub fn get_database_from_remote<P: AsRef<Path> + Debug>(file_path: P) {
    debug!("Getting database from remote");
    let url = "https://www.quaddicted.com/reviews/quaddicted_database.xml".to_string();
    debug!("Database file path: {:?}", file_path);
    match get_remote_file_and_write_to_path(url, &file_path, None) {
        Ok(_) => (),
        Err(e) => {
            // TODO - handle gracefully
            panic!("Something went wrong: {}", e);
        }
    };
}

pub fn get_map_from_remote(
    map_id: &String,
    download_dir: &String,
    progress_sender: Sender<DownloadProgress>,
) {
    trace!("Getting map from remote");
    let url = format!("https://www.quaddicted.com/filebase/{}.zip", map_id);
    debug!("Getting map from url: {}", url);
    let path = format!("{}/{}.zip", download_dir, map_id);
    get_remote_file_and_write_to_path(url, &path, Some(progress_sender))
        .expect("Something went wrong");
}

pub fn get_image_from_remote<P: AsRef<Path> + Debug>(map_id: &String, path: P) {
    trace!("Getting image file from remote");
    let url = format!(
        "https://www.quaddicted.com/reviews/screenshots/{}.jpg",
        map_id
    );
    debug!("Getting image from url: {}", url);
    get_remote_file_and_write_to_path(url, &path, None).expect("Something went wrong");
}

fn get_remote_file_and_write_to_path<P: AsRef<Path> + Debug>(
    url: String,
    path: P,
    sender_opt: Option<Sender<DownloadProgress>>,
) -> Result<(), std::io::Error> {
    let mut response = match get(&url) {
        Ok(r) => r,
        Err(e) => panic!("Failed to get from remote: {}", e),
    };
    let content_length = response.content_length().unwrap();
    let mut file = BufWriter::new(File::create(&path).expect("Couldn't create file"));

    let mut in_bytes = [0; 0x4000];
    let mut total = 0;
    let mut progress_counter = 0.0;
    'byte_reader: loop {
        let file_name = path.as_ref().file_name().unwrap().to_str().unwrap();
        match response.read(&mut in_bytes) {
            Ok(b) => {
                let percent = match total != 0 {
                    true => (total as f64 / content_length as f64),
                    false => 0.0,
                };
                let h_percent = percent * 100.0;
                if h_percent.floor() > progress_counter {
                    progress_counter = h_percent.floor();
                    debug!("progress: {} %", h_percent);
                    send_progress(
                        &sender_opt,
                        DownloadProgress::not_done_dl(percent, file_name),
                    );
                }
                std::io::copy(&mut &in_bytes[0..b], &mut file).expect("Failed to write");
                total += b;
                if total == content_length as usize {
                    send_progress(&sender_opt, DownloadProgress::not_done_dl(100.0, file_name));
                    break 'byte_reader;
                }
            }
            Err(e) => {
                return Err(std::io::Error::new(std::io::ErrorKind::Interrupted, e));
            }
        }
    }
    file.flush()
}

fn send_progress(sender_opt: &Option<Sender<DownloadProgress>>, progress: DownloadProgress) {
    if let Some(sender) = sender_opt {
        sender.send(progress).expect("Failed to send");
    }
}
