use crate::gui_data::GuiData;
use gtk::prelude::*;

pub fn connect_menu_quit(gui_data: &GuiData) {
    let menu_quit = gui_data.menu_quit.clone();
    let window = gui_data.window.clone();
    menu_quit.connect_activate(move |_| {
        window.close();
    });
}
