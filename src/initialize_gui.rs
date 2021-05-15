use crate::gui_data::GuiData;
use crate::{connect_menu_quit, QuakeFile};
use glib::types::Type;
use gtk::prelude::*;

enum Columns {
    Installed = 0,
    Name,
    Title,
    Author,
    Released,
    Rating,
}

pub fn initialize_gui(gui_data: &GuiData, data: &Vec<QuakeFile>) {
    create_list_view(gui_data, data);
    connect_menu_quit(gui_data);
}

fn create_list_view(gui_data: &GuiData, data: &Vec<QuakeFile>) {
    let sw_list = gui_data.sw_list.clone();
    let col_types: [Type; 6] = [
        Type::Bool,
        Type::String,
        Type::String,
        Type::String,
        Type::String,
        Type::String,
    ];
    let list_store = gtk::ListStore::new(&col_types);
    let tree_view = gtk::TreeView::with_model(&list_store);
    populate_list_view(&list_store, data);
    tree_view
        .get_selection()
        .set_mode(gtk::SelectionMode::Single);
    create_tree_view_columns(&tree_view);
    tree_view.set_vexpand(true);
    sw_list.add(&tree_view);
    sw_list.show_all();
}

fn populate_list_view(list_store: &gtk::ListStore, data: &Vec<QuakeFile>) {
    // TODO - hook up real data
    let col_indices = [0, 1, 2, 3, 4, 5];
    for file in data {
        let values: [&dyn ToValue; 6] = [
            &false,
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
    let column = gtk::TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_resizable(false);
    column.add_attribute(&renderer, "active", Columns::Installed as i32);
    tree_view.append_column(&column);
    let renderer = gtk::CellRendererText::new();
    tree_view.append_column(&create_text_column("Id", &renderer, Columns::Name));
    tree_view.append_column(&create_text_column("Title", &renderer, Columns::Title));
    tree_view.append_column(&create_text_column("Author", &renderer, Columns::Author));
    tree_view.append_column(&create_text_column(
        "Released",
        &renderer,
        Columns::Released,
    ));
    tree_view.append_column(&create_text_column("Rating", &renderer, Columns::Rating));
}

fn create_text_column(
    title: &str,
    renderer: &gtk::CellRendererText,
    col: Columns,
) -> gtk::TreeViewColumn {
    let column = gtk::TreeViewColumnBuilder::new()
        .title(title)
        .expand(true)
        .resizable(true)
        .max_width(200)
        .build();
    column.pack_start(renderer, true);
    column.add_attribute(renderer, "text", col as i32);
    column
}
