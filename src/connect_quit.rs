use crate::app::QInjector;
use crate::gui_data::GuiData;
use gtk::prelude::*;
use gtk::AccelGroup;
use log::*;
use std::cell::RefCell;
use std::rc::Rc;

pub fn connect_menu_quit(gui_data: &GuiData) {
    let menu_quit = gui_data.main_menu.menu_quit.clone();
    let window = gui_data.window.clone();
    menu_quit.connect_activate(move |_| {
        window.close();
    });
    add_key_commands(gui_data);
}

pub fn connect_close(gui_data: &GuiData, app: Rc<RefCell<QInjector>>) {
    let window = gui_data.window.clone();
    window.connect_destroy(move |_| {
        debug!("Destroying window");
        app.borrow().write_local_maps();
        app.borrow().write_config();
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
