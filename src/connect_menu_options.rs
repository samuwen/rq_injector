use crate::gui_data::GuiData;
use crate::list_view::populate_list_view;
use crate::request_utils::get_database_from_remote;
use glib::{Continue, MainContext, Receiver, Sender, PRIORITY_DEFAULT};
use gtk::prelude::*;
use gtk::AccelGroup;
use log::*;
use std::thread;

pub fn connect_menu_quit(gui_data: &GuiData) {
    trace!("Initializing quit connection");
    let menu_quit = gui_data.main_menu.menu_quit.clone();
    let window = gui_data.window.clone();
    menu_quit.connect_activate(move |_| {
        debug!("Quit request made");
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
        debug!("Destroying window");
        shared_install_state.borrow().write_to_file();
        shared_config_state.borrow().write_to_file();
    });
}

pub fn connect_reload(gui_data: &GuiData) {
    trace!("Initializing reload connection");
    let menu_reload = gui_data.main_menu.menu_reload.clone();
    let gui_data = gui_data.clone();
    menu_reload.connect_activate(move |_| {
        debug!("Database reload request made");
        let (sender, receiver): (Sender<bool>, Receiver<bool>) =
            MainContext::channel(PRIORITY_DEFAULT);
        thread::Builder::new()
            .name("Reload-0".to_string())
            .spawn(move || {
                get_database_from_remote();
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
