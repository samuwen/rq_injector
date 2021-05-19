use crate::gui_data::GuiData;
use gtk::prelude::*;
use log::*;

pub fn connect_search_event(gui_data: &GuiData) {
    trace!("Initializing search actions");
    let search_entry = gui_data.filter_bar.entry_filter_text.clone();
    let gui_data = gui_data.clone();
    let clear_button = gui_data.filter_bar.btn_clear_filter.clone();
    search_entry.connect_property_text_notify(move |me| {
        let text = me.get_buffer().get_text();
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
    let list = gui_data.list_view.list_store.clone();
    let shared_files_state = gui_data.shared_files_state.clone();
    let shared_install_state = gui_data.shared_install_state.clone();
    let clear_button = gui_data.filter_bar.btn_clear_filter.clone();
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
}

fn compare_text(one: &String, two: &String) -> bool {
    let lc_one = one.to_ascii_lowercase();
    let lc_two = two.to_ascii_lowercase();
    lc_one.contains(&lc_two)
}
