use crate::gui_data::GuiData;
use crate::list_view::populate_list_view;
use crate::request_utils::get_database_from_remote;
use glib::{Continue, MainContext, Receiver, Sender, PRIORITY_DEFAULT};
use gtk::prelude::*;
use gtk::AccelGroup;
use log::*;
use std::fs::{remove_dir, remove_dir_all, remove_file};
use std::path::PathBuf;
use std::thread;

pub fn connect_menu_quit(gui_data: &GuiData) {
    trace!("Initializing quit connection");
    let menu_quit = gui_data.main_menu.menu_quit.clone();
    let window = gui_data.window.clone();
    menu_quit.connect_activate(move |_| {
        info!("Quit request made");
        window.close();
    });
    add_key_commands(gui_data);
}

pub fn connect_close(gui_data: &GuiData) {
    trace!("Initializing close connection");
    let window = gui_data.window.clone();
    let shared_install_state = gui_data.shared_install_state.clone();
    let shared_config_state = gui_data.shared_config_state.clone();
    window.connect_destroy(move |_| {
        info!("Destroying window");
        let config_dir = shared_config_state.borrow().config_dir().clone();
        shared_install_state.borrow().write_to_file(config_dir);
        shared_config_state.borrow().write_to_file();
    });
}

pub fn connect_reload(gui_data: &GuiData) {
    trace!("Initializing reload connection");
    let menu_reload = gui_data.main_menu.menu_reload.clone();
    let gui_data = gui_data.clone();
    let shared_configs = gui_data.shared_config_state.clone();
    menu_reload.connect_activate(move |_| {
        info!("Database reload request made");
        let mut config_dir = shared_configs.borrow().config_dir().clone();
        config_dir.push("database.xml");
        let (sender, receiver): (Sender<bool>, Receiver<bool>) =
            MainContext::channel(PRIORITY_DEFAULT);
        thread::Builder::new()
            .name("Reload-0".to_string())
            .spawn(move || {
                get_database_from_remote(config_dir);
                sender.send(true).expect("Failed to send");
            })
            .expect("Failed to spawn thread");
        let rec_gui_data = gui_data.clone();
        receiver.attach(None, move |result| {
            if result {
                populate_list_view(&rec_gui_data);
            }
            Continue(true)
        });
    });
}

pub fn connect_offline(gui_data: &GuiData) {
    trace!("Initializing offline connection");
    let menu_reload = gui_data.main_menu.menu_reload.clone();
    let menu_offline = gui_data.main_menu.menu_offline.clone();
    let config_state = gui_data.shared_config_state.clone();
    let install_state = gui_data.shared_install_state.clone();
    let tree_view = gui_data.list_view.tree_view.clone();
    let btn_install = gui_data.detail_pane.btn_install.clone();
    menu_offline.connect_toggled(move |me| {
        let active_state = me.get_active();
        let msg = match active_state {
            true => "offline",
            false => "online",
        };
        info!("Switching to {} mode", msg);
        menu_reload.set_sensitive(!active_state);
        match tree_view.get_selection().get_selected() {
            Some((model, iter)) => {
                let string_res: Result<Option<String>, glib::value::GetError> =
                    model.get_value(&iter, 1).get();
                let id_string = string_res.unwrap().unwrap();
                let is_local = install_state.borrow().is_map_installed(&id_string);
                debug!("Install button is now: {}", !active_state && !is_local);
                btn_install.set_sensitive(!active_state && !is_local);
            }
            None => (),
        }
        config_state.borrow_mut().set_is_offline(active_state);
    });
}

pub fn connect_clear_cache(gui_data: &GuiData) {
    trace!("Initializing cache clear");
    let menu_clear_cache = gui_data.main_menu.menu_clear_cache.clone();
    let clear_cache_dialog = gui_data.clear_cache_dialog.dlg_you_sure.clone();
    menu_clear_cache.connect_activate(move |_| {
        info!("Clear cache request made");
        clear_cache_dialog.show_all();
    });
}

pub fn connect_clear_cache_ok(gui_data: &GuiData) {
    trace!("Initializing cache clear ok button");
    let btn_confirm_cache_clear = gui_data.clear_cache_dialog.btn_confirm_clear_cache.clone();
    let clear_cache_dialog = gui_data.clear_cache_dialog.dlg_you_sure.clone();
    let shared_config = gui_data.shared_config_state.clone();
    let config_dir = shared_config.borrow().config_dir().clone();
    let image_cache_dir = shared_config.borrow().image_cache_dir().clone();
    btn_confirm_cache_clear.connect_clicked(move |_| {
        info!("Clear cache ok button pressed");
        debug!("{:?}", config_dir);
        clear_cache_dialog.hide();
        remove_file_with_name(&config_dir, "config.xml");
        remove_file_with_name(&config_dir, "database.xml");
        remove_file_with_name(&config_dir, "installedMaps.xml");
        match remove_dir_all(&image_cache_dir) {
            Ok(_) => debug!("Removed images directory"),
            Err(e) => error!("Error removing images directory: {}", e),
        };
        match remove_dir(&config_dir) {
            Ok(_) => debug!("Removed cache directory"),
            Err(e) => error!("Error removing cache directory: {}", e),
        };
        std::process::exit(0);
    });
}

pub fn connect_clear_cache_cancel(gui_data: &GuiData) {
    trace!("Initializing clear cache cancel button");
    let btn_cancel_clear_cache = gui_data.clear_cache_dialog.btn_cancel_clear_cache.clone();
    let clear_cache_dialog = gui_data.clear_cache_dialog.dlg_you_sure.clone();
    btn_cancel_clear_cache.connect_clicked(move |_| {
        clear_cache_dialog.hide();
    });
}

pub fn remove_file_with_name(config_dir: &PathBuf, name: &str) {
    let mut file_path = config_dir.clone();
    file_path.push(name);
    match remove_file(&file_path) {
        Ok(_) => debug!("Removed file: {}", name),
        Err(e) => error!("Failed to remove file: {} because {}", name, e),
    }
}

fn add_key_commands(gui_data: &GuiData) {
    let menu_quit = gui_data.main_menu.menu_quit.clone();
    let window = gui_data.window.clone();
    let accel_group = AccelGroup::new();
    window.add_accel_group(&accel_group);
    let (key, modifier) = gtk::accelerator_parse("<Primary>Q");
    menu_quit.add_accelerator(
        "activate",
        &accel_group,
        key,
        modifier,
        gtk::AccelFlags::VISIBLE,
    );
}
