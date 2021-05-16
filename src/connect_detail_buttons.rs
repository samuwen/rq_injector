use crate::app::QInjector;
use crate::gui_data::GuiData;
use crate::installer::Installer;
use gio::prelude::*;
use gtk::prelude::*;
use log::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::thread;

pub fn connect_install_map(gui_data: &GuiData, app: Rc<RefCell<QInjector>>) {
    let (sender, receiver): (glib::Sender<Installer>, glib::Receiver<Installer>) =
        glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    let install_button = gui_data.detail_pane.btn_install.clone();
    let rec_gui_data = gui_data.clone();
    let shared_install_state = rec_gui_data.shared_install_state.clone();
    receiver.attach(None, move |installer| {
        debug!("Done installing");
        set_installed_state(
            &rec_gui_data,
            app.clone(),
            true,
            installer.get_path_string(),
        );
        *shared_install_state.borrow_mut() = installer;
        glib::Continue(false)
    });
    let con_gui_data = gui_data.clone();
    install_button.connect_clicked(move |_| {
        trace!("Install button clicked");
        // TODO - look for map in download dir and bypass pinging remote
        let map_id = get_selected_map_id(&con_gui_data);
        let path_string = get_current_path_string(&con_gui_data);
        let sender = sender.clone();
        thread::spawn(move || {
            let mut installer = Installer::new().path_string(path_string);
            installer.install_map(map_id);
            sender.send(installer);
        });
    });
}

pub fn connect_uninstall_map(gui_data: &GuiData, app: Rc<RefCell<QInjector>>) {
    let button = gui_data.detail_pane.btn_uninstall.clone();
    let gui_data = gui_data.clone();
    button.connect_clicked(move |_| {
        trace!("Uninstall button clicked");
        let id = get_selected_map_id(&gui_data);
        app.borrow_mut().uninstall_map(&id);
        set_installed_state(&gui_data, app.clone(), false, String::new());
    });
}

pub fn connect_play_button(gui_data: &GuiData, app: Rc<RefCell<QInjector>>) {
    let button = gui_data.detail_pane.btn_play.clone();
    let gui_data = gui_data.clone();
    let dropdown = gui_data.detail_pane.dropdown.clone();
    let output_dialog = gui_data.output_dialog.dlg_output.clone();
    let output_text = gui_data.output_dialog.txt_output.clone();
    button.connect_clicked(move |_| {
        let model = dropdown.get_model().unwrap();
        let iter = dropdown.get_active_iter().unwrap();
        let string_res: Result<Option<String>, glib::value::GetError> =
            model.get_value(&iter, 0).get();
        let start_map = string_res.unwrap().unwrap();
        let id = get_selected_map_id(&gui_data);
        let result = app.borrow().play_quake_map(&id, &start_map).unwrap();
        let text: String = result
            .stdout
            .iter()
            .map(|b| match b.is_ascii() {
                true => *b as char,
                false => ' ',
            })
            .collect();
        output_text.get_buffer().unwrap().set_text(&text);
        output_dialog.show_all();
    });
}

fn set_installed_state(
    gui_data: &GuiData,
    app: Rc<RefCell<QInjector>>,
    is_local: bool,
    path_string: String,
) {
    let current_path_string = get_current_path_string(gui_data);
    if path_string == current_path_string {
        let install_button = gui_data.detail_pane.btn_install.clone();
        let uninstall_button = gui_data.detail_pane.btn_uninstall.clone();
        let play_button = gui_data.detail_pane.btn_play.clone();
        install_button.set_sensitive(!is_local);
        uninstall_button.set_sensitive(is_local);
        play_button.set_sensitive(is_local);
    }
    update_list(&gui_data, is_local, path_string);
}

fn update_list(gui_data: &GuiData, is_local: bool, path_string: String) {
    let list = gui_data.list_view.list_store.clone();
    let (_, iter) = get_list_selection(gui_data, path_string);
    list.set_value(&iter, 0, &is_local.to_value());
}

fn get_selected_map_id(gui_data: &GuiData) -> String {
    let (model, iter) = get_current_list_selection(gui_data);
    let string_res: Result<Option<String>, glib::value::GetError> = model.get_value(&iter, 1).get();
    string_res.unwrap().unwrap()
}

fn get_current_list_selection(gui_data: &GuiData) -> (gtk::TreeModel, gtk::TreeIter) {
    let tree_view = gui_data.list_view.tree_view.clone();
    tree_view.get_selection().get_selected().unwrap()
}

fn get_list_selection(gui_data: &GuiData, path_string: String) -> (gtk::TreeModel, gtk::TreeIter) {
    let tree_view = gui_data.list_view.tree_view.clone();
    let model = tree_view.get_model().unwrap();
    let iter = model.get_iter_from_string(&path_string).unwrap();
    (model, iter)
}

fn get_current_path_string(gui_data: &GuiData) -> String {
    let (model, iter) = get_current_list_selection(gui_data);
    model.get_string_from_iter(&iter).unwrap().to_string()
}
