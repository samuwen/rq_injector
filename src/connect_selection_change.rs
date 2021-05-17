use crate::gui_data::GuiData;
use bytes::Bytes;
use gdk_pixbuf::Pixbuf;
use glib::{Continue, MainContext, Receiver, Sender, PRIORITY_DEFAULT};
use gtk::prelude::*;
use log::*;
use reqwest::blocking::{ClientBuilder, Response};
use std::fs::{write, File};
use std::sync::atomic::{AtomicU8, Ordering};
use std::thread;

static THREAD_COUNTER: AtomicU8 = AtomicU8::new(0);

pub fn connect_selection_change(gui_data: &GuiData, tree_view: &gtk::TreeView) {
    let detail_pane = gui_data.detail_pane.clone();
    let shared_install_state = gui_data.shared_install_state.clone();
    let shared_files_state = gui_data.shared_files_state.clone();
    let (sender, receiver): (Sender<(String, String)>, Receiver<(String, String)>) =
        MainContext::channel(PRIORITY_DEFAULT);
    receiver.attach(None, move |result| {
        let (path, id_string) = result;
        let pixbuf = Pixbuf::from_file_at_size(path, 200, 200).unwrap();
        let file = shared_files_state
            .borrow()
            .iter()
            .find(|f| f.id() == &id_string)
            .unwrap()
            .clone();
        let is_local = shared_install_state.borrow().is_map_installed(&id_string);
        detail_pane.update(&file, pixbuf, is_local);
        THREAD_COUNTER.fetch_sub(1, Ordering::Relaxed);
        Continue(true)
    });
    tree_view.get_selection().connect_changed(move |sel| {
        let (model, iter) = sel.get_selected().unwrap();
        let string_res: Result<Option<String>, glib::value::GetError> =
            model.get_value(&iter, 1).get();
        let id_string = string_res.unwrap().unwrap();
        let sender = sender.clone();
        thread::Builder::new()
            .name(format!("select-{}", THREAD_COUNTER.load(Ordering::Relaxed)))
            .spawn(move || {
                THREAD_COUNTER.fetch_add(1, Ordering::Relaxed);
                let path = load_map_image(&id_string);
                match sender.send((path, id_string)) {
                    Ok(_) => (),
                    Err(e) => {
                        error!("{}", e);
                    }
                }
            })
            .expect("Failed to spawn select thread");
    });
}

pub fn load_map_image(id: &String) -> String {
    let path = format!("images/{}.jpg", id);
    debug!("Attempting to load image at: {:?}", path);
    let file_opt = File::open(&path);
    match file_opt {
        Ok(_) => path,
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
                write_file_to_disk(&bytes, &path);
                return path;
            } else {
                panic!("Failed to get dem bytes");
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

fn write_file_to_disk(bytes: &Bytes, path: &String) -> bool {
    debug!("Writing out to {}", path);
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
