use crate::gui_data::GuiData;
use crate::initializable::Initializable;
use crate::locales::Locale;
use crate::quake_file::{initialize_data, Files};
use chrono::NaiveDate;
use glib::Type;
use glib::{Continue, MainContext, Receiver, Sender, PRIORITY_DEFAULT};
use gtk::prelude::*;
use gtk::{Builder, ListStore, ScrolledWindow, TreeIter, TreeModel, TreeView};
use log::*;
use std::thread;

#[derive(Clone)]
pub struct ListView {
    pub sw_list: ScrolledWindow,
    pub list_store: ListStore,
    pub tree_view: TreeView,
}

impl ListView {
    pub fn create_from_builder(builder: &Builder) -> Self {
        let col_types: [Type; 7] = [
            Type::Bool,
            Type::String,
            Type::String,
            Type::String,
            Type::String,
            gdk_pixbuf::Pixbuf::static_type(),
            Type::U8,
        ];
        let sw_list: ScrolledWindow = builder
            .get_object("sw_list")
            .expect("Failed to get sw_list");
        let list_store = gtk::ListStore::new(&col_types);
        let tree_view = gtk::TreeView::with_model(&list_store);
        Self {
            sw_list,
            list_store,
            tree_view,
        }
    }

    pub fn initialize(&self, gui_data: &GuiData) {
        let released_index = gtk::SortColumn::Index(Columns::Released as u32);
        self.list_store.set_sort_func(released_index, date_sort_fn);
        let rating_index = gtk::SortColumn::Index(Columns::Rating as u32);
        self.list_store.set_sort_func(rating_index, rating_sort_fn);
        self.tree_view
            .get_selection()
            .set_mode(gtk::SelectionMode::Single);
        create_tree_view_columns(&self.tree_view, gui_data);
        self.tree_view.set_vexpand(true);
        self.sw_list.add(&self.tree_view);
        self.sw_list.show_all();
    }

    fn display_data_in_list(&self, gui_data: &GuiData) {
        let shared_install_state = gui_data.shared_install_state.clone();
        let shared_files_state = gui_data.shared_files_state.clone();
        let shared_images = gui_data.shared_images.clone();
        let shared_config_state = gui_data.shared_config_state.clone();
        let col_indices = [0, 1, 2, 3, 4, 5, 6];
        for file in shared_files_state.borrow().iter() {
            let rating = match u8::from_str_radix(file.rating(), 10) {
                Ok(r) => r,
                Err(_) => 0,
            };
            let rating_image = &shared_images.borrow()[rating as usize];
            let date_format = shared_config_state.borrow().get_date_format();
            let naive_date = NaiveDate::parse_from_str(file.date(), "%d.%m.%Y").unwrap();
            let date = naive_date.format(&date_format).to_string();
            let values: [&dyn ToValue; 7] = [
                &shared_install_state.borrow().is_map_installed(file.id()),
                file.id(),
                file.title(),
                file.author(),
                &date,
                rating_image,
                &rating,
            ];
            self.list_store
                .set(&self.list_store.append(), &col_indices, &values);
        }
    }

    fn set_col_title(&self, column: Columns, title: &String) {
        let col = self
            .tree_view
            .get_column(column as i32)
            .expect("No column!");
        col.set_title(title);
    }
}

impl Initializable for ListView {
    fn init_text(&self, locale: &Locale) {
        self.set_col_title(Columns::Name, locale.id_column_name());
        self.set_col_title(Columns::Title, locale.title_column_name());
        self.set_col_title(Columns::Author, locale.author_column_name());
        self.set_col_title(Columns::Released, locale.released_column_name());
        self.set_col_title(Columns::Rating, locale.rating_column_name());
    }
}

pub fn populate_list_view(gui_data: &GuiData) {
    trace!("Populating list view");
    let (sender, receiver): (Sender<Files>, Receiver<Files>) =
        MainContext::channel(PRIORITY_DEFAULT);
    let rec_gui_data = gui_data.clone();
    let list_view = gui_data.list_view.clone();
    let shared_configs = gui_data.shared_config_state.clone();
    let config_dir = shared_configs.borrow().config_dir().clone();
    thread::Builder::new()
        .name("List-0".to_string())
        .spawn(move || {
            let files = initialize_data(config_dir);
            sender.send(files).expect("Failed to send");
        })
        .expect("Failed to spawn thread");
    receiver.attach(None, move |files| {
        let rec_gui_data = rec_gui_data.clone();
        let quake_files = files.files().clone();
        let shared_files_state = rec_gui_data.shared_files_state.clone();
        *shared_files_state.borrow_mut() = quake_files;
        list_view.display_data_in_list(&rec_gui_data);
        Continue(true)
    });
}

fn create_tree_view_columns(tree_view: &gtk::TreeView, gui_data: &GuiData) {
    let renderer = gtk::CellRendererToggle::new();
    let installed_column = gtk::TreeViewColumn::new();
    installed_column.pack_start(&renderer, true);
    installed_column.set_resizable(false);
    installed_column.add_attribute(&renderer, "active", Columns::Installed as i32);
    installed_column.set_sort_column_id(Columns::Installed as i32);

    let config = gui_data.shared_config_state.clone();

    let renderer = gtk::CellRendererText::new();
    let id_column = create_text_column(
        config.borrow().current_locale().id_column_name(),
        &renderer,
        Columns::Name,
    );
    let title_column = create_text_column(
        config.borrow().current_locale().title_column_name(),
        &renderer,
        Columns::Title,
    );
    let author_column = create_text_column(
        config.borrow().current_locale().author_column_name(),
        &renderer,
        Columns::Author,
    );
    let released_column = create_text_column(
        config.borrow().current_locale().released_column_name(),
        &renderer,
        Columns::Released,
    );
    let title = config
        .borrow()
        .current_locale()
        .rating_column_name()
        .to_owned();
    let rating_column = create_rating_column(title);

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

fn create_rating_column(title: String) -> gtk::TreeViewColumn {
    let col_int = Columns::Rating as i32;
    let renderer = gtk::CellRendererPixbuf::new();
    let column = gtk::TreeViewColumnBuilder::new()
        .max_width(200)
        .title(&title)
        .clickable(true)
        .sort_column_id(col_int)
        .expand(true)
        .resizable(true)
        .build();
    column.pack_start(&renderer, true);
    column.add_attribute(&renderer, "pixbuf", col_int);
    column
}

fn rating_sort_fn(model: &TreeModel, row_1: &TreeIter, row_2: &TreeIter) -> std::cmp::Ordering {
    let rating_1: u8 = model
        .get_value(row_1, Columns::RatingSort as i32)
        .get()
        .unwrap()
        .unwrap();
    let rating_2: u8 = model
        .get_value(row_2, Columns::RatingSort as i32)
        .get()
        .unwrap()
        .unwrap();
    rating_1.cmp(&rating_2)
}

fn date_sort_fn(model: &TreeModel, row_1: &TreeIter, row_2: &TreeIter) -> std::cmp::Ordering {
    let date_1 = get_date(model.get_value(row_1, Columns::Released as i32));
    let date_2 = get_date(model.get_value(row_2, Columns::Released as i32));
    date_1.cmp(&date_2)
}

fn get_date(val: glib::Value) -> NaiveDate {
    let valid_formats = vec!["%d.%m.%Y", "%d-%m-%Y", "%m-%d-%Y", "%m.%d.%Y"];
    let val: String = val.get().unwrap().unwrap();
    for format in valid_formats {
        let date_res = NaiveDate::parse_from_str(&val, format);
        if let Ok(date) = date_res {
            return date;
        }
    }
    panic!("Invalid date format for value {}", val);
}

enum Columns {
    Installed = 0,
    Name,
    Title,
    Author,
    Released,
    Rating,
    RatingSort, // for sneaky hidden column
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
            6 => String::from("RatingSort"),
            _ => panic!("Dude cmon"),
        }
    }
}
