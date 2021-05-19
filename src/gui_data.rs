use crate::config_dialog::ConfigDialog;
use crate::configuration::{Configuration, LocalMaps};
use crate::detail_pane::DetailPane;
use crate::filter_bar::FilterBar;
use crate::list_view::ListView;
use crate::main_menu::MainMenu;
use crate::output_dialog::OutputDialog;
use crate::quake_file::{initialize_data, QuakeFile};
use gdk_pixbuf::Pixbuf;
use gtk::prelude::*;
use gtk::{Builder, Window};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct GuiData {
    pub glade_src: String,
    pub builder: Builder,

    pub window: Window,
    pub main_menu: MainMenu,
    pub filter_bar: FilterBar,
    pub detail_pane: DetailPane,
    pub list_view: ListView,
    pub config_dialog: ConfigDialog,
    pub output_dialog: OutputDialog,

    pub shared_install_state: Rc<RefCell<LocalMaps>>,
    pub shared_files_state: Rc<RefCell<Vec<QuakeFile>>>,
    pub shared_config_state: Rc<RefCell<Configuration>>,
}

impl GuiData {
    pub fn new() -> Self {
        let glade_src = include_str!("../rq.glade").to_string();
        let builder = Builder::from_string(&glade_src);

        let window: gtk::Window = builder
            .get_object("window_main")
            .expect("Failed to get window_main");
        let pixbuf = Pixbuf::from_file_at_size("resources/injector64.png", 64, 64).unwrap();
        window.set_icon(Some(&pixbuf));
        let main_menu = MainMenu::create_from_builder(&builder);
        let filter_bar = FilterBar::create_from_builder(&builder);
        let detail_pane = DetailPane::create_from_builder(&builder);
        let list_view = ListView::create_from_builder(&builder);
        let config_dialog = ConfigDialog::create_from_builder(&builder);
        let output_dialog = OutputDialog::create_from_builder(&builder);
        let shared_install_state = Rc::new(RefCell::new(LocalMaps::new()));
        let quake_files = initialize_data();
        let shared_files_state = Rc::new(RefCell::new(quake_files));
        let shared_config_state = Rc::new(RefCell::new(Configuration::new()));
        window.set_position(gtk::WindowPosition::CenterAlways);
        window.show_all();
        {
            let window = window.clone();
            window.connect_delete_event(move |_, _| {
                gtk::main_quit();
                gtk::Inhibit(false)
            });
        }

        Self {
            glade_src,
            builder,
            window,
            main_menu,
            filter_bar,
            detail_pane,
            list_view,
            config_dialog,
            output_dialog,
            shared_install_state,
            shared_files_state,
            shared_config_state,
        }
    }
}
