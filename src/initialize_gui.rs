use crate::app::QInjector;
use crate::connect_config_dialog;
use crate::connect_detail_buttons;
use crate::connect_output_dialog;
use crate::connect_quit::*;
use crate::gui_data::GuiData;
use crate::quake_file::*;
use gtk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

enum Columns {
    Installed = 0,
    Name,
    Title,
    Author,
    Released,
    Rating,
}

pub fn initialize_gui(gui_data: &GuiData, app: Rc<RefCell<QInjector>>) {
    create_list_view(gui_data, app.clone());
    connect_menu_quit(gui_data);
    connect_close(gui_data, app.clone());

    initialize_dialog_connectors(gui_data, app.clone());
    initialize_detail_buttons(gui_data, app);
    initialize_output_dialog(gui_data);
}

fn create_list_view(gui_data: &GuiData, app: Rc<RefCell<QInjector>>) {
    let sw_list = gui_data.list_view.sw_list.clone();
    let list_store = gui_data.list_view.list_store.clone();
    let tree_view = gui_data.list_view.tree_view.clone();
    populate_list_view(&list_store, app.borrow().files());
    tree_view
        .get_selection()
        .set_mode(gtk::SelectionMode::Single);
    create_tree_view_columns(&tree_view);
    tree_view.set_vexpand(true);
    handle_selection_change(gui_data, &tree_view, app);
    sw_list.add(&tree_view);
    sw_list.show_all();
}

fn handle_selection_change(
    gui_data: &GuiData,
    tree_view: &gtk::TreeView,
    app: Rc<RefCell<QInjector>>,
) {
    let detail_pane = gui_data.detail_pane.clone();
    tree_view.get_selection().connect_changed(move |sel| {
        let (model, iter) = sel.get_selected().unwrap();
        let string_res: Result<Option<String>, glib::value::GetError> =
            model.get_value(&iter, 1).get();
        let id_string = string_res.unwrap().unwrap();
        let borrow = app.borrow();
        let file = borrow
            .files()
            .iter()
            .find(|f| f.id() == &id_string)
            .unwrap();
        let pixbuf = app.borrow().load_map_image(id_string);
        detail_pane.update(&file, pixbuf);
    });
}

fn populate_list_view(list_store: &gtk::ListStore, data: &Vec<QuakeFile>) {
    let col_indices = [0, 1, 2, 3, 4, 5];
    for file in data {
        let values: [&dyn ToValue; 6] = [
            file.installed_locally(),
            file.id(),
            file.title(),
            file.author(),
            file.date(),
            file.rating(),
        ];
        list_store.set(&list_store.append(), &col_indices, &values);
    }
}

// TODO - dates don't sort right. need to figure out a way to get that working
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
    column
}

fn initialize_dialog_connectors(gui_data: &GuiData, app: Rc<RefCell<QInjector>>) {
    connect_config_dialog::connect_activate(gui_data, app.clone());
    connect_config_dialog::connect_cancel(gui_data);
    connect_config_dialog::connect_ok(gui_data, app.clone());
    connect_config_dialog::connect_selects(gui_data);
    connect_config_dialog::connect_response(gui_data, app);
}

fn initialize_detail_buttons(gui_data: &GuiData, app: Rc<RefCell<QInjector>>) {
    connect_detail_buttons::connect_install_map(gui_data, app.clone());
    connect_detail_buttons::connect_uninstall_map(gui_data, app.clone());
    connect_detail_buttons::connect_play_button(gui_data, app.clone());
}

fn initialize_output_dialog(gui_data: &GuiData) {
    connect_output_dialog::connect_ok(gui_data);
}
