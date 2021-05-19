use crate::connect_config_dialog;
use crate::connect_detail_buttons;
use crate::connect_output_dialog;
use crate::connect_quit::*;
use crate::connect_search_event;
use crate::connect_selection_change;
use crate::gui_data::GuiData;
use gtk::prelude::*;
use gtk::{TreeIter, TreeModel};
use log::*;

enum Columns {
    Installed = 0,
    Name,
    Title,
    Author,
    Released,
    Rating,
}

impl Columns {
    fn get_from_id(id: i32) -> String {
        match id {
            0 => String::from("Installed"),
            1 => String::from("Name"),
            2 => String::from("Title"),
            3 => String::from("Author"),
            4 => String::from("Released"),
            5 => String::from("Rating"),
            _ => panic!("Dude cmon"),
        }
    }
}

pub fn initialize_gui() {
    let gui_data = GuiData::new();
    create_list_view(&gui_data);
    connect_menu_quit(&gui_data);
    connect_close(&gui_data);
    connect_search_event::connect_search_event(&gui_data);

    initialize_dialog_connectors(&gui_data);
    initialize_detail_buttons(&gui_data);
    initialize_output_dialog(&gui_data);
}

fn create_list_view(gui_data: &GuiData) {
    let sw_list = gui_data.list_view.sw_list.clone();
    let list_store = gui_data.list_view.list_store.clone();
    let tree_view = gui_data.list_view.tree_view.clone();
    populate_list_view(&list_store, gui_data);
    let c_index = gtk::SortColumn::Index(Columns::Released as u32);
    list_store.set_sort_func(c_index, date_sort_fn);
    tree_view
        .get_selection()
        .set_mode(gtk::SelectionMode::Single);
    create_tree_view_columns(&tree_view);
    tree_view.set_vexpand(true);
    connect_selection_change::connect_selection_change(gui_data, &tree_view);
    sw_list.add(&tree_view);
    sw_list.show_all();
}

fn populate_list_view(list_store: &gtk::ListStore, gui_data: &GuiData) {
    let shared_install_state = gui_data.shared_install_state.clone();
    let col_indices = [0, 1, 2, 3, 4, 5];
    let shared_files_state = gui_data.shared_files_state.clone();
    for file in shared_files_state.borrow().iter() {
        let values: [&dyn ToValue; 6] = [
            &shared_install_state.borrow().is_map_installed(file.id()),
            file.id(),
            file.title(),
            file.author(),
            file.date(),
            file.rating(),
        ];
        list_store.set(&list_store.append(), &col_indices, &values);
    }
}

fn create_tree_view_columns(tree_view: &gtk::TreeView) {
    let renderer = gtk::CellRendererToggle::new();
    let installed_column = gtk::TreeViewColumn::new();
    installed_column.pack_start(&renderer, true);
    installed_column.set_resizable(false);
    installed_column.add_attribute(&renderer, "active", Columns::Installed as i32);
    installed_column.set_sort_column_id(Columns::Installed as i32);

    let renderer = gtk::CellRendererText::new();
    let id_column = create_text_column("Id", &renderer, Columns::Name);
    let title_column = create_text_column("Title", &renderer, Columns::Title);
    let author_column = create_text_column("Author", &renderer, Columns::Author);
    let released_column = create_text_column("Released", &renderer, Columns::Released);
    let rating_column = create_text_column("Rating", &renderer, Columns::Rating);

    tree_view.append_column(&installed_column);
    tree_view.append_column(&id_column);
    tree_view.append_column(&title_column);
    tree_view.append_column(&author_column);
    tree_view.append_column(&released_column);
    tree_view.append_column(&rating_column);
}

fn create_text_column(
    title: &str,
    renderer: &gtk::CellRendererText,
    col: Columns,
) -> gtk::TreeViewColumn {
    let col_int = col as i32;
    let column = gtk::TreeViewColumnBuilder::new()
        .title(title)
        .expand(true)
        .resizable(true)
        .max_width(200)
        .clickable(true)
        .sort_column_id(col_int)
        .build();
    column.pack_start(renderer, true);
    column.add_attribute(renderer, "text", col_int);
    column.connect_clicked(move |me| {
        let sort_id = me.get_sort_column_id();
        debug!("Sorting column: {}", Columns::get_from_id(sort_id));
    });
    column
}

fn initialize_dialog_connectors(gui_data: &GuiData) {
    connect_config_dialog::connect_activate(gui_data);
    connect_config_dialog::connect_cancel(gui_data);
    connect_config_dialog::connect_ok(gui_data);
    connect_config_dialog::connect_selects(gui_data);
}

fn initialize_detail_buttons(gui_data: &GuiData) {
    connect_detail_buttons::connect_install_map(gui_data);
    connect_detail_buttons::connect_uninstall_map(gui_data);
    connect_detail_buttons::connect_play_button(gui_data);
}

fn initialize_output_dialog(gui_data: &GuiData) {
    connect_output_dialog::connect_ok(gui_data);
}

fn date_sort_fn(model: &TreeModel, row_1: &TreeIter, row_2: &TreeIter) -> std::cmp::Ordering {
    let date_1 = get_date(model.get_value(row_1, Columns::Released as i32));
    let date_2 = get_date(model.get_value(row_2, Columns::Released as i32));
    date_1.cmp(&date_2)
}

fn get_date(val: glib::Value) -> chrono::NaiveDate {
    let val: String = val.get().unwrap().unwrap();
    let date_res = chrono::NaiveDate::parse_from_str(&val, "%d.%m.%Y");
    match date_res {
        Ok(d) => d,
        Err(e) => panic!("{}", e),
    }
}
