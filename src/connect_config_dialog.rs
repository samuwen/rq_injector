use crate::app::QInjector;
use crate::gui_data::GuiData;
use gtk::prelude::*;
use log::*;

pub fn connect_activate(gui_data: &GuiData) {
    trace!("Initializing config activation");
    let engine_config = gui_data.main_menu.menu_engine_configuration.clone();
    let dialog = gui_data.config_dialog.dlg_config.clone();
    engine_config.connect_activate(move |_| {
        dialog.show_all();
    });
}

pub fn connect_cancel(gui_data: &GuiData) {
    trace!("Initializing config cancel button");
    let dialog = gui_data.config_dialog.dlg_config.clone();
    gui_data.config_dialog.btn_cancel.connect_clicked(move |_| {
        trace!("Config cancel clicked");
        dialog.hide();
    });
}

pub fn connect_ok(gui_data: &GuiData, app: &QInjector) {
    trace!("Initializing config ok button");
    let dialog = gui_data.config_dialog.clone();
    let button = dialog.btn_ok.clone();
    let app = app.clone();
    button.connect_clicked(move |_| {
        trace!("Config ok clicked");
        let mut app = app.clone();
        let quake_exe = dialog.ent_quake_exe.get_buffer().get_text();
        let quake_dir = dialog.ent_quake_dir.get_buffer().get_text();
        let download_dir = dialog.ent_download_dir.get_buffer().get_text();
        let hip_checked = dialog.chk_hipnotic.get_active();
        let rogue_checked = dialog.chk_rogue.get_active();
        app.set_quake_exe(quake_exe);
        app.set_quake_dir(quake_dir);
        app.set_download_dir(download_dir);
        app.set_hip_check(hip_checked);
        app.set_rogue_check(rogue_checked);
        dialog.dlg_config.clone().hide();
    });
}
