use crate::initializable::Initializable;
use crate::locales::Locale;
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
    pub menu_file: MenuItem,
    pub menu_config: MenuItem,
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
        let menu_file: MenuItem = builder
            .get_object("menu_file")
            .expect("Failed to get menu_file");
        let menu_config: MenuItem = builder
            .get_object("menu_config")
            .expect("Failed to get menu_config");
        Self {
            menu_bar,
            menu_reload,
            menu_check_for_installed,
            menu_offline,
            menu_quit,
            menu_engine_configuration,
            menu_clear_cache,
            menu_file,
            menu_config,
        }
    }

    pub fn init_states(&self, is_offline: bool) {
        self.menu_reload.set_sensitive(!is_offline);
        self.menu_offline.set_active(is_offline);
    }
}

impl Initializable for MainMenu {
    fn init_text(&self, locale: &Locale) {
        self.menu_check_for_installed
            .set_label(locale.check_installed_menu_option());
        self.menu_clear_cache
            .set_label(locale.clear_cache_menu_option());
        self.menu_engine_configuration
            .set_label(locale.configuration_menu_option());
        self.menu_offline.set_label(locale.offline_menu_option());
        self.menu_quit.set_label(locale.quit_menu_option());
        self.menu_reload
            .set_label(locale.reload_database_menu_option());
        self.menu_file.set_label(locale.file_menu_name());
        self.menu_config.set_label(locale.configuration_menu_name());
    }
}
