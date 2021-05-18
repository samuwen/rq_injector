use crate::gui_data::GuiData;
use gtk::prelude::*;
use gtk::AccelGroup;
use log::*;

pub fn connect_menu_quit(gui_data: &GuiData) {
    let menu_quit = gui_data.main_menu.menu_quit.clone();
    let window = gui_data.window.clone();
    menu_quit.connect_activate(move |_| {
        window.close();
    });
    add_key_commands(gui_data);
}

pub fn connect_close(gui_data: &GuiData) {
    let window = gui_data.window.clone();
    let shared_install_state = gui_data.shared_install_state.clone();
    window.connect_destroy(move |_| {
        debug!("Destroying window");
        shared_install_state.borrow().write_to_file();
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
