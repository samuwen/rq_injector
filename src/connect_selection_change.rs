use crate::gui_data::GuiData;
use crate::image_loader::ImageLoader;
use gdk_pixbuf::Pixbuf;
use glib::{Continue, MainContext, Receiver, Sender, PRIORITY_DEFAULT};
use gtk::prelude::*;
use log::*;
use std::sync::atomic::{AtomicU8, Ordering};
use std::thread;

static THREAD_COUNTER: AtomicU8 = AtomicU8::new(0);

pub fn connect_selection_change(gui_data: &GuiData, tree_view: &gtk::TreeView) {
    let detail_pane = gui_data.detail_pane.clone();
    let shared_install_state = gui_data.shared_install_state.clone();
    let shared_files_state = gui_data.shared_files_state.clone();
    let shared_config_state = gui_data.shared_config_state.clone();
    let (sender, receiver): (Sender<ImageLoader>, Receiver<ImageLoader>) =
        MainContext::channel(PRIORITY_DEFAULT);
    let rec_tree_view = tree_view.clone();
    let rec_detail_pane = detail_pane.clone();
    receiver.attach(None, move |image_loader| {
        let pixbuf = Pixbuf::from_file_at_size(image_loader.path(), 200, 200).unwrap();
        match rec_tree_view.get_selection().get_selected() {
            Some((model, iter)) => {
                let current_path_string = model.get_string_from_iter(&iter).unwrap().to_string();
                if &current_path_string == image_loader.path_string() {
                    rec_detail_pane.update_image(pixbuf);
                }
            }
            None => (),
        };
        THREAD_COUNTER.fetch_sub(1, Ordering::Relaxed);
        Continue(true)
    });
    tree_view
        .get_selection()
        .connect_changed(move |sel| match sel.get_selected() {
            Some((model, iter)) => {
                let string_res: Result<Option<String>, glib::value::GetError> =
                    model.get_value(&iter, 1).get();
                let id_string = string_res.unwrap().unwrap();
                trace!("Changed selection to {}", id_string);
                let path_string = model.get_string_from_iter(&iter).unwrap().to_string();
                let file = shared_files_state
                    .borrow()
                    .iter()
                    .find(|f| f.id() == &id_string)
                    .unwrap()
                    .clone();
                let is_local = shared_install_state.borrow().is_map_installed(&id_string);
                let is_offline = *shared_config_state.borrow().is_offline();
                let image_dir = shared_config_state.borrow().image_cache_dir().clone();
                detail_pane.update(&file, is_local, is_offline);
                let sender = sender.clone();
                thread::Builder::new()
                    .name(format!("select-{}", THREAD_COUNTER.load(Ordering::Relaxed)))
                    .spawn(move || {
                        THREAD_COUNTER.fetch_add(1, Ordering::Relaxed);
                        let mut image_loader = ImageLoader::new(id_string, path_string);
                        image_loader.load_map_image(is_offline, image_dir);
                        match sender.send(image_loader) {
                            Ok(_) => (),
                            Err(e) => {
                                error!("{}", e);
                            }
                        }
                    })
                    .expect("Failed to spawn select thread");
            }
            None => {
                trace!("Clearing selected");
                detail_pane.clear();
            }
        });
}
