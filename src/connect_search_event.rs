use crate::gui_data::GuiData;
use gtk::prelude::*;
use log::*;

pub fn connect_search_event(gui_data: &GuiData) {
    trace!("Initializing search actions");
    let search_entry = gui_data.filter_bar.entry_filter_text.clone();
    let gui_data = gui_data.clone();
    let clear_button = gui_data.filter_bar.btn_clear_filter.clone();
    search_entry.connect_property_text_notify(move |me| {
        // we have text in the box
        let text = me.get_buffer().get_text().trim().to_owned();
        update_list(&gui_data, text);
    });
    clear_button.connect_clicked(move |me| {
        trace!("Clear button clicked");
        search_entry.get_buffer().set_text("");
        me.set_sensitive(false);
    });
}

fn update_list(gui_data: &GuiData, text: String) {
    trace!("Updating list with text: {}", text);
    let tree_view = gui_data.list_view.tree_view.clone();
    let selection = tree_view.get_selection();
    // if something is selected
    if let Some((model, iter)) = selection.get_selected() {
        // record the internal representation of the selected item
        let path_string = model.get_string_from_iter(&iter).unwrap().to_string();
        change_list_data(gui_data, text);
        // if it is still shown, re-select it
        if let Some(iter) = model.get_iter_from_string(&path_string) {
            debug!("Valid path, re-selecting node");
            selection.select_iter(&iter);
        }
    } else {
        // nothing is selected
        change_list_data(gui_data, text);
    }
}

fn change_list_data(gui_data: &GuiData, text: String) {
    let list = gui_data.list_view.list_store.clone();
    let shared_files_state = gui_data.shared_files_state.clone();
    let shared_install_state = gui_data.shared_install_state.clone();
    let clear_button = gui_data.filter_bar.btn_clear_filter.clone();
    let tree_view = gui_data.list_view.tree_view.clone();
    let selection = tree_view.get_selection();
    // Set it so nothing can be selected while we update the list
    selection.set_mode(gtk::SelectionMode::None);
    selection.unselect_all();
    clear_button.set_sensitive(text.len() > 0);
    let col_indices = [0, 1, 2, 3, 4, 5];
    list.clear();
    shared_files_state.borrow().iter().for_each(|file| {
        let valid_id = compare_text(file.id(), &text);
        let valid_title = compare_text(file.title(), &text);
        let valid_author = compare_text(file.author(), &text);
        let valid_date = compare_text(file.date(), &text);
        if valid_id || valid_title || valid_author || valid_date {
            let values: [&dyn ToValue; 6] = [
                &shared_install_state.borrow().is_map_installed(file.id()),
                file.id(),
                file.title(),
                file.author(),
                file.date(),
                file.rating(),
            ];
            list.set(&list.append(), &col_indices, &values);
        }
    });
    // done updating list, so lets re-enable selectability
    selection.set_mode(gtk::SelectionMode::Single);
}

fn compare_text(one: &String, two: &String) -> bool {
    let lc_one = one.to_ascii_lowercase();
    let lc_two = two.to_ascii_lowercase();
    lc_one.contains(&lc_two)
}
