use crate::download_progress::DownloadProgress;
use crate::game_player::*;
use crate::gui_data::GuiData;
use crate::initializable::Initializable;
use crate::installer::Installer;
use crate::quake_file::QuakeFile;
use gio::prelude::*;
use glib::{Continue, MainContext, Receiver, Sender, PRIORITY_DEFAULT};
use gtk::prelude::*;
use log::*;
use rand::prelude::*;
use std::process::Output;
use std::sync::atomic::{AtomicU8, Ordering};
use std::thread;

static THREAD_COUNTER: AtomicU8 = AtomicU8::new(0);

pub fn connect_install_map(gui_data: &GuiData) {
    let (sender, receiver): (Sender<Installer>, Receiver<Installer>) =
        MainContext::channel(PRIORITY_DEFAULT);
    let (progress_sender, progress_receiver): (
        Sender<DownloadProgress>,
        Receiver<DownloadProgress>,
    ) = MainContext::channel(PRIORITY_DEFAULT);
    let button = gui_data.detail_pane.btn_install.clone();
    let detail_pane = gui_data.detail_pane.clone();
    let rec_gui_data = gui_data.clone();
    let shared_install_state = rec_gui_data.shared_install_state.clone();
    receiver.attach(None, move |installer| {
        release_thread_name();
        set_installed_state(&rec_gui_data, true, &installer);
        let map_pack = installer.installed_map_pack().clone().unwrap();
        shared_install_state.borrow_mut().add_map(map_pack);
        Continue(true)
    });
    progress_receiver.attach(None, move |dl_progress| {
        detail_pane.update_progress_bar(&dl_progress.file_name(), *dl_progress.percent());
        Continue(true)
    });
    let con_gui_data = gui_data.clone();

    button.connect_clicked(move |_| {
        trace!("Install button clicked");
        let map_id = get_selected_map_id(&con_gui_data)
            .expect("Nothing was selected, so this shouldn't happen");
        install_map(
            &con_gui_data,
            progress_sender.clone(),
            sender.clone(),
            map_id,
        );
    });
}

pub fn connect_uninstall_map(gui_data: &GuiData) {
    let (sender, receiver): (Sender<Installer>, Receiver<Installer>) =
        MainContext::channel(PRIORITY_DEFAULT);
    let button = gui_data.detail_pane.btn_uninstall.clone();
    let rec_gui_data = gui_data.clone();
    let shared_install_state = rec_gui_data.shared_install_state.clone();
    let shared_config_state = rec_gui_data.shared_config_state.clone();
    let rec_shared_install_state = shared_install_state.clone();
    receiver.attach(None, move |installer| {
        release_thread_name();
        set_installed_state(&rec_gui_data, false, &installer);
        let map_id = installer.map_id();
        rec_shared_install_state.borrow_mut().remove_map(map_id);
        Continue(true)
    });

    let con_gui_data = gui_data.clone();
    button.connect_clicked(move |_| {
        trace!("Uninstall button clicked");
        let map_id = get_selected_map_id(&con_gui_data)
            .expect("Nothing was selected, so this shouldn't happen");
        let path_string = get_current_path_string(&con_gui_data)
            .expect("Nothing was selected, so this shouldn't happen")
            .to_string();
        let download_dir = shared_config_state.borrow().download_dir().to_owned();
        let quake_dir = shared_config_state.borrow().quake_dir().to_owned();

        let sender = sender.clone();
        let thread_name = get_thread_name("uninstall");
        thread::Builder::new()
            .name(thread_name)
            .spawn(move || {
                let installer = Installer::new()
                    .with_download_dir(download_dir)
                    .with_quake_dir(quake_dir)
                    .with_path_string(path_string)
                    .with_map_id(map_id);
                installer.uninstall_map();
                sender.send(installer).expect("Couldn't send");
            })
            .expect("Failed to spawn install thread");
    });
}

pub fn connect_play_button(gui_data: &GuiData) {
    let (sender, receiver): (Sender<Output>, Receiver<Output>) =
        MainContext::channel(PRIORITY_DEFAULT);
    let button = gui_data.detail_pane.btn_play.clone();
    let gui_data = gui_data.clone();
    let dropdown = gui_data.detail_pane.dropdown.clone();
    let output_dialog = gui_data.output_dialog.clone();
    let dlg_output = output_dialog.dlg_output.clone();
    let output_text = gui_data.output_dialog.txt_output.clone();
    let shared_files_state = gui_data.shared_files_state.clone();
    let shared_config_state = gui_data.shared_config_state.clone();
    let con_shared_config_state = gui_data.shared_config_state.clone();
    receiver.attach(None, move |result| {
        let text: String = result
            .stdout
            .iter()
            .map(|b| match b.is_ascii() {
                true => *b as char,
                false => ' ',
            })
            .collect();
        output_text.get_buffer().unwrap().set_text(&text);
        output_dialog.init_text(shared_config_state.borrow().current_locale());
        dlg_output.show_all();
        Continue(true)
    });
    button.connect_clicked(move |_| {
        let model = dropdown.get_model().unwrap();
        let start_map = match dropdown.get_active_iter() {
            Some(iter) => {
                let string_res: Result<Option<String>, glib::value::GetError> =
                    model.get_value(&iter, 0).get();
                Some(string_res.unwrap().unwrap())
            }
            None => None,
        };
        let map_id =
            get_selected_map_id(&gui_data).expect("Nothing was selected, so this shouldn't happen");
        let quake_exe = con_shared_config_state.borrow().quake_exe().to_owned();
        let quake_dir = con_shared_config_state.borrow().quake_dir().to_owned();
        let command_line_opt = shared_files_state
            .borrow()
            .iter()
            .find(|file| file.id() == &map_id)
            .unwrap()
            .tech_info()
            .command_line()
            .to_owned();
        let sender = sender.clone();
        let thread_name = get_thread_name("play");
        thread::Builder::new()
            .name(thread_name)
            .spawn(move || {
                let game_player = GamePlayerBuilder::default()
                    .quake_exe(quake_exe)
                    .quake_dir(quake_dir)
                    .map_id(map_id)
                    .start_map(start_map)
                    .command_line(command_line_opt)
                    .build()
                    .unwrap();
                let out = game_player.play_quake_map();
                sender.send(out).expect("Couldn't send");
            })
            .expect("Failed to spawn play thread");
    });
}

// Here to share the install map logic
pub fn connect_install_random_map(gui_data: &GuiData) {
    let (sender, receiver): (Sender<Installer>, Receiver<Installer>) =
        MainContext::channel(PRIORITY_DEFAULT);
    let (progress_sender, progress_receiver): (
        Sender<DownloadProgress>,
        Receiver<DownloadProgress>,
    ) = MainContext::channel(PRIORITY_DEFAULT);
    let button = gui_data.filter_bar.btn_install_random.clone();
    let detail_pane = gui_data.detail_pane.clone();
    let rec_gui_data = gui_data.clone();
    let shared_install_state = rec_gui_data.shared_install_state.clone();
    receiver.attach(None, move |installer| {
        release_thread_name();
        set_installed_state(&rec_gui_data, true, &installer);
        let map_pack = installer.installed_map_pack().clone().unwrap();
        shared_install_state.borrow_mut().add_map(map_pack);
        Continue(true)
    });
    progress_receiver.attach(None, move |dl_progress| {
        detail_pane.update_progress_bar(&dl_progress.file_name(), *dl_progress.percent());
        Continue(true)
    });
    let con_gui_data = gui_data.clone();

    button.connect_clicked(move |_| {
        trace!("Install random button clicked");
        let map_list_state = con_gui_data.shared_files_state.clone();
        let map_list = map_list_state.borrow();
        let installed_map_list = con_gui_data.shared_install_state.clone();
        let uninstalled_maps: Vec<&QuakeFile> = map_list
            .iter()
            .filter(|map| !installed_map_list.borrow().is_map_installed(map.id()))
            .collect();
        let mut rng = thread_rng();
        let map_choice = rng.gen_range(0..uninstalled_maps.len());
        let map_id = uninstalled_maps
            .get(map_choice)
            .expect("Map choice index is out of bounds")
            .id();
        select_map_by_id(&con_gui_data, &map_id);
        install_map(
            &con_gui_data,
            progress_sender.clone(),
            sender.clone(),
            map_id.to_owned(),
        );
    });
}

fn install_map(
    gui_data: &GuiData,
    progress_sender: Sender<DownloadProgress>,
    install_sender: Sender<Installer>,
    map_id: String,
) {
    let path_string = get_path_string_from_id(gui_data, &map_id);
    let tree_view = gui_data.list_view.tree_view.clone();
    let column = tree_view
        .get_column(0)
        .expect("No zero column for some reason");
    tree_view.scroll_to_cell(
        Some(&gtk::TreePath::from_string(&path_string)),
        Some(&column),
        false,
        0.0,
        0.5,
    );
    let shared_config_state = gui_data.shared_config_state.clone();
    let download_dir = shared_config_state.borrow().download_dir().to_owned();
    let quake_dir = shared_config_state.borrow().quake_dir().to_owned();
    let mut detail_pane = gui_data.detail_pane.clone();
    detail_pane.add_progress_bar(&map_id);
    let progress_sender = progress_sender.clone();
    let thread_name = get_thread_name("install");
    thread::Builder::new()
        .name(thread_name)
        .spawn(move || {
            let mut installer = Installer::new()
                .with_download_dir(download_dir)
                .with_quake_dir(quake_dir)
                .with_map_id(map_id)
                .with_path_string(path_string);
            installer.install_map(progress_sender);
            install_sender.send(installer).expect("Couldn't send");
        })
        .expect("Failed to spawn install thread");
}

fn set_installed_state(gui_data: &GuiData, is_local: bool, installer: &Installer) {
    let config_state = gui_data.shared_config_state.clone();
    if let Some(current_path_string) = get_current_path_string(gui_data) {
        if installer.path_string() == &current_path_string {
            let install_button = gui_data.detail_pane.btn_install.clone();
            let uninstall_button = gui_data.detail_pane.btn_uninstall.clone();
            let play_button = gui_data.detail_pane.btn_play.clone();
            install_button.set_sensitive(!is_local && !config_state.borrow().is_offline());
            uninstall_button.set_sensitive(is_local);
            play_button.set_sensitive(is_local);
        }
    }
    update_list(&gui_data, is_local, installer.path_string());
}

fn update_list(gui_data: &GuiData, is_local: bool, path_string: &String) {
    let list = gui_data.list_view.list_store.clone();
    match get_list_selection(gui_data, &path_string) {
        Some((_, iter)) => {
            list.set_value(&iter, 0, &is_local.to_value());
        }
        None => (),
    }
}

fn get_selected_map_id(gui_data: &GuiData) -> Option<String> {
    match get_current_list_selection(gui_data) {
        Some((model, iter)) => {
            let string_res: Result<Option<String>, glib::value::GetError> =
                model.get_value(&iter, 1).get();
            string_res.unwrap()
        }
        None => None,
    }
}

fn get_current_list_selection(gui_data: &GuiData) -> Option<(gtk::TreeModel, gtk::TreeIter)> {
    let tree_view = gui_data.list_view.tree_view.clone();
    tree_view.get_selection().get_selected()
}

fn get_list_selection(
    gui_data: &GuiData,
    path_string: &String,
) -> Option<(gtk::TreeModel, gtk::TreeIter)> {
    let tree_view = gui_data.list_view.tree_view.clone();
    let model = tree_view
        .get_model()
        .expect("No model in the tree view. Something catastrophic happened");
    trace!("Getting path from path string: {}", path_string);
    match model.get_iter_from_string(path_string) {
        Some(iter) => Some((model, iter)),
        None => None,
    }
}

fn get_current_path_string(gui_data: &GuiData) -> Option<glib::GString> {
    match get_current_list_selection(gui_data) {
        Some((model, iter)) => model.get_string_from_iter(&iter),
        None => None,
    }
}

fn get_thread_name(name: &str) -> String {
    let name = format!("{}-{}", name, THREAD_COUNTER.load(Ordering::Relaxed));
    THREAD_COUNTER.fetch_add(1, Ordering::Relaxed);
    trace!("Thread counter: {:?}", THREAD_COUNTER);
    name
}

fn release_thread_name() {
    THREAD_COUNTER.fetch_sub(1, Ordering::Relaxed);
    trace!("Thread counter: {:?}", THREAD_COUNTER);
}

fn get_path_string_from_id(gui_data: &GuiData, id: &String) -> String {
    let tree_view = gui_data.list_view.tree_view.clone();
    let tree_model = tree_view.get_model().unwrap();
    let mut path_string = String::new();
    tree_model.foreach(|model, _path, iter| {
        let string_res: Result<Option<String>, glib::value::GetError> =
            model.get_value(&iter, 1).get();
        let map_id = string_res.unwrap().unwrap();
        let found = &map_id == id;
        if found {
            path_string = model
                .get_string_from_iter(&iter)
                .expect("Path string all fucked")
                .to_string();
        }
        found
    });
    path_string
}

fn select_map_by_id(gui_data: &GuiData, id: &String) {
    let tree_view = gui_data.list_view.tree_view.clone();
    let tree_model = tree_view.get_model().expect("No model in tree");
    let selection = tree_view.get_selection();
    tree_model.foreach(|model, path, iter| {
        let string_res: Result<Option<String>, glib::value::GetError> =
            model.get_value(&iter, 1).get();
        let map_id = string_res.unwrap().unwrap();
        let found = &map_id == id;
        if found {
            selection.select_path(&path);
        }
        found
    })
}
