use crate::app::QInjector;
use crate::gui_data::GuiData;
use gtk::prelude::*;
use gtk::{Button, Dialog, Entry, FileChooserAction, ResponseType};
use log::*;
use std::cell::RefCell;
use std::rc::Rc;

pub fn connect_activate(gui_data: &GuiData, app: Rc<RefCell<QInjector>>) {
    trace!("Initializing config activation");
    let engine_config = gui_data.main_menu.menu_engine_configuration.clone();
    let dialog = gui_data.config_dialog.dlg_config.clone();
    let quake_dir_entry = gui_data.config_dialog.ent_quake_dir.clone();
    let quake_exe_entry = gui_data.config_dialog.ent_quake_exe.clone();
    let download_dir_entry = gui_data.config_dialog.ent_download_dir.clone();
    engine_config.connect_activate(move |_| {
        let borrow = app.borrow();
        quake_dir_entry.set_text(borrow.config().quake_dir());
        download_dir_entry.set_text(borrow.config().download_dir());
        quake_exe_entry.set_text(borrow.config().quake_exe());
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

pub fn connect_ok(gui_data: &GuiData, app: Rc<RefCell<QInjector>>) {
    trace!("Initializing config ok button");
    let dialog = gui_data.config_dialog.clone();
    let button = dialog.btn_ok.clone();
    button.connect_clicked(move |_| {
        trace!("Config ok clicked");
        let mut borrow = app.borrow_mut();
        let quake_exe = dialog.ent_quake_exe.get_buffer().get_text();
        let quake_dir = dialog.ent_quake_dir.get_buffer().get_text();
        let download_dir = dialog.ent_download_dir.get_buffer().get_text();
        let hip_checked = dialog.chk_hipnotic.get_active();
        let rogue_checked = dialog.chk_rogue.get_active();
        borrow.set_quake_exe(quake_exe);
        borrow.set_quake_dir(quake_dir);
        borrow.set_download_dir(download_dir);
        borrow.set_hip_check(hip_checked);
        borrow.set_rogue_check(rogue_checked);
        dialog.dlg_config.clone().hide();
    });
}

pub fn connect_selects(gui_data: &GuiData) {
    trace!("Initializing select buttons");
    let dialog = gui_data.config_dialog.clone();
    connect_input(
        dialog.btn_quake_dir.clone(),
        dialog.ent_quake_dir.clone(),
        dialog.dlg_config.clone(),
        "quake folder".to_string(),
        FileChooserAction::SelectFolder,
    );
    connect_input(
        dialog.btn_download_dir.clone(),
        dialog.ent_download_dir.clone(),
        dialog.dlg_config.clone(),
        "download folder".to_string(),
        FileChooserAction::SelectFolder,
    );
    connect_input(
        dialog.btn_quake_exe.clone(),
        dialog.ent_quake_exe.clone(),
        dialog.dlg_config.clone(),
        "quake executable".to_string(),
        FileChooserAction::Open,
    );
}

pub fn connect_response(gui_data: &GuiData, app: Rc<RefCell<QInjector>>) {
    trace!("Initializing file chooser response handler");
    let dialog = gui_data.config_dialog.dlg_config.clone();
    let quake_dir_input = gui_data.config_dialog.ent_quake_dir.clone();
    let download_dir_input = gui_data.config_dialog.ent_download_dir.clone();
    let quake_exe_input = gui_data.config_dialog.ent_quake_exe.clone();
    dialog.connect_response(move |me, res_type| {
        let app = app.clone();
        let mut borrow = app.borrow_mut();
        match res_type {
            ResponseType::Ok => {
                borrow.set_quake_dir(quake_dir_input.get_text().to_string());
                borrow.set_quake_exe(quake_exe_input.get_text().to_string());
                borrow.set_download_dir(download_dir_input.get_text().to_string());
                me.hide();
            }
            ResponseType::Cancel => {
                me.hide();
            }
            _ => (),
        }
    });
}

fn connect_input(btn: Button, ent: Entry, dlg: Dialog, name: String, action: FileChooserAction) {
    btn.connect_clicked(move |_| {
        handle_file_dialog(&dlg, &ent, &name, action);
    });
}

fn handle_file_dialog(
    dialog: &gtk::Dialog,
    input: &gtk::Entry,
    name: &str,
    action: FileChooserAction,
) {
    let input_msg = format!("Setting {} input to: ", name);
    let pick_msg = format!("Please select {}", name);
    let file_dialog = gtk::FileChooserDialog::with_buttons(
        Some(&pick_msg),
        Some(dialog),
        action,
        &[
            ("_Cancel", ResponseType::Cancel),
            ("_Open", ResponseType::Accept),
        ],
    );
    match file_dialog.run() {
        ResponseType::Cancel => file_dialog.hide(),
        ResponseType::Accept => {
            let f = file_dialog.get_filename().unwrap();
            input.set_text(f.to_str().unwrap());
            debug!("{}", format!("{}{:?}", input_msg, f));
            file_dialog.hide();
        }
        _ => (),
    }
}
