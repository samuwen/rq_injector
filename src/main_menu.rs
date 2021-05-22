use gtk::prelude::*;
use gtk::{Builder, CheckMenuItem, MenuBar, MenuItem};

#[derive(Clone)]
pub struct MainMenu {
    pub menu_bar: MenuBar,
    pub menu_reload: MenuItem,
    pub menu_check_for_installed: MenuItem,
    pub menu_offline: CheckMenuItem,
    pub menu_clear_cache: MenuItem,
    pub menu_quit: MenuItem,
    pub menu_engine_configuration: MenuItem,
}

impl MainMenu {
    pub fn create_from_builder(builder: &Builder) -> Self {
        let menu_bar: MenuBar = builder
            .get_object("menu_bar")
            .expect("Failed to get menu_bar");
        let menu_reload: MenuItem = builder
            .get_object("menu_reload")
            .expect("Failed to get menu_reload");
        let menu_check_for_installed: MenuItem = builder
            .get_object("menu_check_for_installed")
            .expect("Failed to get menu_check_for_installed");
        let menu_clear_cache: MenuItem = builder
            .get_object("menu_clear_cache")
            .expect("Failed to get menu_clear_cache");
        let menu_offline: CheckMenuItem = builder
            .get_object("menu_offline")
            .expect("Failed to get menu_offline");
        let menu_quit: MenuItem = builder
            .get_object("menu_quit")
            .expect("Failed to get menu_quit");
        let menu_engine_configuration: MenuItem = builder
            .get_object("menu_engine_configuration")
            .expect("Failed to get menu_engine_configuration");
        Self {
            menu_bar,
            menu_reload,
            menu_check_for_installed,
            menu_offline,
            menu_quit,
            menu_engine_configuration,
            menu_clear_cache,
        }
    }

    pub fn init_states(&self, is_offline: bool) {
        self.menu_reload.set_sensitive(!is_offline);
        self.menu_offline.set_active(is_offline);
    }
}
