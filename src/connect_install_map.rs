use crate::app::QInjector;
use crate::gui_data::GuiData;
use gio::prelude::*;
use gtk::prelude::*;
use log::*;

pub fn connect_install_map(gui_data: &GuiData, app: &QInjector) {
    trace!("Install button clicked");
    let install_button = gui_data.detail_pane.btn_install.clone();
    let gui_data = gui_data.clone();
    let app = app.clone();
    install_button.connect_clicked(move |_| {
        let mut app = app.clone(); // needed to capture in closure
        let name_option = app.start_map_install();
        if let Some((files_to_extract, dupe_file_names)) = name_option {
            match dupe_file_names.len() > 0 {
                true => {
                    info!("Dupe files: {:?}", dupe_file_names);
                    // TODO - implement duplicate file handling
                }
                false => {
                    let all_installed = app.extract_data_from_zip(&files_to_extract);
                    match all_installed {
                        true => {
                            set_installed_state(&mut app, gui_data.clone());
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

fn set_installed_state(app: &mut QInjector, gui_data: GuiData) {
    let install_button = gui_data.detail_pane.btn_install.clone();
    let uninstall_button = gui_data.detail_pane.btn_uninstall.clone();
    let play_button = gui_data.detail_pane.btn_play.clone();
    update_list(&gui_data);

    install_button.set_sensitive(false);
    uninstall_button.set_sensitive(true);
    play_button.set_sensitive(true);
    app.update_current_file_download_status(true);
}

fn update_list(gui_data: &GuiData) {
    let list = gui_data.list_view.list_store.clone();
    let tree_view = gui_data.list_view.tree_view.clone();
    let (_, iter) = tree_view.get_selection().get_selected().unwrap();
    list.set_value(&iter, 0, &true.to_value());
}
