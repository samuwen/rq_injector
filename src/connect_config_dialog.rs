use crate::gui_data::GuiData;
use gtk::prelude::*;
use gtk::{Button, Dialog, Entry, FileChooserAction, ResponseType};
use log::*;

pub fn connect_activate(gui_data: &GuiData) {
    trace!("Initializing config activation");
    let engine_config = gui_data.main_menu.menu_engine_configuration.clone();
    let dialog = gui_data.config_dialog.dlg_config.clone();
    let quake_dir_entry = gui_data.config_dialog.ent_quake_dir.clone();
    let quake_exe_entry = gui_data.config_dialog.ent_quake_exe.clone();
    let download_dir_entry = gui_data.config_dialog.ent_download_dir.clone();
    let shared_config_state = gui_data.shared_config_state.clone();
    engine_config.connect_activate(move |_| {
        let borrow = shared_config_state.borrow();
        quake_dir_entry.set_text(borrow.quake_dir());
        download_dir_entry.set_text(borrow.download_dir());
        quake_exe_entry.set_text(borrow.quake_exe());
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

pub fn connect_ok(gui_data: &GuiData) {
    trace!("Initializing config ok button");
    let dialog = gui_data.config_dialog.clone();
    let button = dialog.btn_ok.clone();
    let shared_config_state = gui_data.shared_config_state.clone();
    button.connect_clicked(move |_| {
        trace!("Config ok clicked");
        let mut borrow = shared_config_state.borrow_mut();
        let quake_exe = dialog.ent_quake_exe.get_buffer().get_text();
        let quake_dir = dialog.ent_quake_dir.get_buffer().get_text();
        let download_dir = dialog.ent_download_dir.get_buffer().get_text();
        let hip_checked = dialog.chk_hipnotic.get_active();
        let rogue_checked = dialog.chk_rogue.get_active();
        borrow.set_quake_exe(quake_exe);
        borrow.set_quake_dir(quake_dir);
        borrow.set_download_dir(download_dir);
        borrow.set_hip_installed(hip_checked);
        borrow.set_rogue_installed(rogue_checked);
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
