use crate::app::QInjector;
use crate::gui_data::GuiData;
use gtk::prelude::*;

pub fn connect_activate_config(gui_data: &GuiData) {
    let engine_config = gui_data.main_menu.menu_engine_configuration.clone();
    let dialog = gui_data.config_dialog.dlg_config.clone();
    engine_config.connect_activate(move |_| {
        dialog.show_all();
    });
}

pub fn connect_
