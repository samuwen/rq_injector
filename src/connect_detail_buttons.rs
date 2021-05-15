use crate::app::QInjector;
use crate::gui_data::GuiData;
use gio::prelude::*;
use gtk::prelude::*;
use log::*;
use std::cell::RefCell;
use std::rc::Rc;

pub fn connect_install_map(gui_data: &GuiData, app: Rc<RefCell<QInjector>>) {
    let install_button = gui_data.detail_pane.btn_install.clone();
    let gui_data = gui_data.clone();
    install_button.connect_clicked(move |_| {
        trace!("Install button clicked");
        // TODO - look for map in download dir and bypass pinging remote
        let map_id = get_selected_map_id(&gui_data);
        let name_option = app.borrow_mut().start_map_install(&map_id);
        if let Some((files_to_extract, dupe_file_names)) = name_option {
            match dupe_file_names.len() > 0 {
                true => {
                    info!("Dupe files: {:?}", dupe_file_names);
                    // TODO - implement duplicate file handling
                }
                false => {
                    let all_installed = app
                        .borrow_mut()
                        .extract_data_from_zip(&files_to_extract, &map_id);
                    match all_installed {
                        true => {
                            set_installed_state(&gui_data, app.clone(), true);
                        }
                        false => {
                            warn!("Something went wrong in installation");
                        }
                    }
                }
            }
        }
    });
}

pub fn connect_uninstall_map(gui_data: &GuiData, app: Rc<RefCell<QInjector>>) {
    let button = gui_data.detail_pane.btn_uninstall.clone();
    let gui_data = gui_data.clone();
    button.connect_clicked(move |_| {
        trace!("Uninstall button clicked");
        let id = get_selected_map_id(&gui_data);
        app.borrow_mut().uninstall_map(&id);
        set_installed_state(&gui_data, app.clone(), false);
    });
}

fn set_installed_state(gui_data: &GuiData, app: Rc<RefCell<QInjector>>, is_local: bool) {
    let install_button = gui_data.detail_pane.btn_install.clone();
    let uninstall_button = gui_data.detail_pane.btn_uninstall.clone();
    let play_button = gui_data.detail_pane.btn_play.clone();
    update_list(&gui_data, is_local);
    let map_id = get_selected_map_id(gui_data);

    install_button.set_sensitive(!is_local);
    uninstall_button.set_sensitive(is_local);
    play_button.set_sensitive(is_local);
    app.borrow_mut()
        .update_current_file_download_status(is_local, &map_id);
}

fn update_list(gui_data: &GuiData, is_local: bool) {
    let list = gui_data.list_view.list_store.clone();
    let (_, iter) = get_current_list_selection(gui_data);
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
