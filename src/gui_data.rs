use gtk::prelude::*;
use gtk::{
    Builder, Button, CheckMenuItem, Entry, Image, Label, MenuBar, MenuItem, ScrolledWindow, Window,
};

pub struct GuiData {
    pub glade_src: String,
    pub builder: Builder,

    pub window: Window,

    // menu
    pub menu_bar: MenuBar,
    pub menu_reload: MenuItem,
    pub menu_check_for_installed: MenuItem,
    pub menu_offline: CheckMenuItem,
    pub menu_quit: MenuItem,
    pub menu_engine_configuration: MenuItem,

    // filter bar
    pub entry_filter_text: Entry,
    pub btn_clear_filter: Button,
    pub btn_install_random: Button,

    // details
    pub lbl_title: Label,
    pub img_current_map: Image,
    pub lbl_description: Label,
    pub btn_install: Button,
    pub btn_uninstall: Button,
    pub btn_play: Button,
    pub sw_details: ScrolledWindow,

    pub sw_list: ScrolledWindow,
}

impl GuiData {
    pub fn new() -> Self {
        let glade_src = include_str!("../rq.glade").to_string();
        let builder = Builder::from_string(&glade_src);

        let window: gtk::Window = builder
            .get_object("window_main")
            .expect("Failed to get window_main");
        let menu_bar: MenuBar = builder
            .get_object("menu_bar")
            .expect("Failed to get menu_bar");
        let menu_reload: MenuItem = builder
            .get_object("menu_reload")
            .expect("Failed to get menu_reload");
        let menu_check_for_installed: MenuItem = builder
            .get_object("menu_check_for_installed")
            .expect("Failed to get menu_check_for_installed");
        let menu_offline: CheckMenuItem = builder
            .get_object("menu_offline")
            .expect("Failed to get menu_offline");
        let menu_quit: MenuItem = builder
            .get_object("menu_quit")
            .expect("Failed to get menu_quit");
        let menu_engine_configuration: MenuItem = builder
            .get_object("menu_engine_configuration")
            .expect("Failed to get menu_engine_configuration");
        let entry_filter_text: Entry = builder
            .get_object("entry_filter_text")
            .expect("Failed to get entry_filter_text");
        let btn_clear_filter: Button = builder
            .get_object("btn_clear_filter")
            .expect("Failed to get btn_clear_filter");
        let btn_install_random: Button = builder
            .get_object("btn_install_random")
            .expect("Failed to get btn_install_random");
        let lbl_description: Label = builder
            .get_object("lbl_description")
            .expect("Failed to get lbl_description");
        let img_current_map: Image = builder
            .get_object("img_current_map")
            .expect("Failed to get img_current_map");
        let lbl_title: Label = builder
            .get_object("lbl_title")
            .expect("Failed to get lbl_title");
        let btn_install: Button = builder
            .get_object("btn_install")
            .expect("Failed to get btn_install");
        let btn_uninstall: Button = builder
            .get_object("btn_uninstall")
            .expect("Failed to get btn_uninstall");
        let btn_play: Button = builder
            .get_object("btn_play")
            .expect("Failed to get btn_play");
        let sw_list: ScrolledWindow = builder
            .get_object("sw_list")
            .expect("Failed to get sw_list");
        let sw_details: ScrolledWindow = builder
            .get_object("sw_details")
            .expect("Failed to get sw_detail");
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
            menu_bar,
            menu_reload,
            menu_check_for_installed,
            menu_offline,
            menu_quit,
            menu_engine_configuration,
            entry_filter_text,
            btn_clear_filter,
            btn_install_random,
            lbl_title,
            lbl_description,
            img_current_map,
            btn_play,
            btn_install,
            btn_uninstall,
            sw_list,
            sw_details,
        }
    }
}
