use crate::gui_data::GuiData;
use crate::initializable::Initializable;
use crate::locales::Locale;
use gtk::prelude::*;
use gtk::{Button, Dialog, Entry, FileChooserAction, ResponseType};
use log::*;

pub fn connect_activate(gui_data: &GuiData) {
    trace!("Initializing config activation");
    let engine_config = gui_data.main_menu.menu_engine_configuration.clone();
    let dialog = gui_data.config_dialog.clone();
    let shared_config = gui_data.shared_config_state.clone();
    let gui_data = gui_data.clone();
    engine_config.connect_activate(move |_| {
        dialog.init_text(shared_config.borrow().current_locale());
        dialog.show(gui_data.shared_config_state.clone());
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
    let detail_pane = gui_data.detail_pane.clone();
    let filter_bar = gui_data.filter_bar.clone();
    let main_menu = gui_data.main_menu.clone();
    let list_view = gui_data.list_view.clone();
    button.connect_clicked(move |_| {
        trace!("Config ok clicked");
        let mut borrow = shared_config_state.borrow_mut();
        let quake_exe = dialog
            .engine_config_tab
            .ent_quake_exe
            .get_buffer()
            .get_text();
        let quake_dir = dialog
            .engine_config_tab
            .ent_quake_dir
            .get_buffer()
            .get_text();
        let download_dir = dialog
            .engine_config_tab
            .ent_download_dir
            .get_buffer()
            .get_text();
        let hip_checked = dialog.engine_config_tab.chk_hipnotic.get_active();
        let rogue_checked = dialog.engine_config_tab.chk_rogue.get_active();
        let language = dialog
            .personal_config_tab
            .dropdown_choose_language
            .get_active_text()
            .unwrap();
        let date_format = dialog
            .personal_config_tab
            .dropdown_choose_dateformat
            .get_active_text()
            .unwrap();
        borrow.set_quake_exe(quake_exe);
        borrow.set_quake_dir(quake_dir);
        borrow.set_download_dir(download_dir);
        borrow.set_hip_installed(hip_checked);
        borrow.set_rogue_installed(rogue_checked);
        borrow.set_date_format(date_format.to_string());
        borrow.set_language(language.to_string());
        detail_pane.init_text(borrow.current_locale());
        filter_bar.init_text(borrow.current_locale());
        main_menu.init_text(borrow.current_locale());
        list_view.init_text(borrow.current_locale());
        dialog.dlg_config.clone().hide();
    });
}

pub fn connect_selects(gui_data: &GuiData) {
    trace!("Initializing select buttons");
    let dialog = gui_data.config_dialog.clone();
    let config = gui_data.shared_config_state.clone();
    connect_input(
        dialog.engine_config_tab.btn_quake_dir.clone(),
        dialog.engine_config_tab.ent_quake_dir.clone(),
        dialog.dlg_config.clone(),
        config
            .borrow()
            .current_locale()
            .config_dialog_quake_dir_text()
            .to_string(),
        FileChooserAction::SelectFolder,
        config.borrow().current_locale().clone(),
    );
    connect_input(
        dialog.engine_config_tab.btn_download_dir.clone(),
        dialog.engine_config_tab.ent_download_dir.clone(),
        dialog.dlg_config.clone(),
        config
            .borrow()
            .current_locale()
            .config_dialog_download_dir_text()
            .to_string(),
        FileChooserAction::SelectFolder,
        config.borrow().current_locale().clone(),
    );
    connect_input(
        dialog.engine_config_tab.btn_quake_exe.clone(),
        dialog.engine_config_tab.ent_quake_exe.clone(),
        dialog.dlg_config.clone(),
        config
            .borrow()
            .current_locale()
            .config_dialog_quake_exe_text()
            .to_string(),
        FileChooserAction::Open,
        config.borrow().current_locale().clone(),
    );
}

fn connect_input(
    btn: Button,
    ent: Entry,
    dlg: Dialog,
    name: String,
    action: FileChooserAction,
    locale: Locale,
) {
    btn.connect_clicked(move |_| {
        let locale = locale.clone();
        handle_file_dialog(&dlg, &ent, &name, action, locale);
    });
}

fn handle_file_dialog(
    dialog: &gtk::Dialog,
    input: &gtk::Entry,
    name: &str,
    action: FileChooserAction,
    locale: Locale,
) {
    let input_msg = format!("Setting {} input to: ", name);
    // string interpolation: Rust style
    let title_string = locale.file_chooser_title().clone();
    let mut split_vec: Vec<&str> = title_string.split("{}").collect();
    split_vec.insert(1, name);
    let pick_msg: String = split_vec.join("");
    let file_dialog = gtk::FileChooserDialog::with_buttons(
        Some(&pick_msg),
        Some(dialog),
        action,
        &[
            (locale.universal_cancel_button(), ResponseType::Cancel),
            (locale.universal_ok_button(), ResponseType::Accept),
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
