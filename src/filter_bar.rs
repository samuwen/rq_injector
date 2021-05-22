use crate::initializable::Initializable;
use crate::locales::Locale;
use gtk::prelude::*;
use gtk::{Builder, Button, Entry};

#[derive(Clone)]
pub struct FilterBar {
    pub entry_filter_text: Entry,
    pub btn_clear_filter: Button,
    pub btn_install_random: Button,
}

impl FilterBar {
    pub fn create_from_builder(builder: &Builder) -> Self {
        let entry_filter_text: Entry = builder
            .get_object("entry_filter_text")
            .expect("Failed to get entry_filter_text");
        let btn_clear_filter: Button = builder
            .get_object("btn_clear_filter")
            .expect("Failed to get btn_clear_filter");
        let btn_install_random: Button = builder
            .get_object("btn_install_random")
            .expect("Failed to get btn_install_random");
        Self {
            entry_filter_text,
            btn_clear_filter,
            btn_install_random,
        }
    }
}

impl Initializable for FilterBar {
    fn init_text(&self, locale: &Locale) {
        self.btn_clear_filter
            .set_label(locale.clear_filter_text_button());
        self.btn_install_random
            .set_label(locale.install_random_map_button());
        self.entry_filter_text
            .set_placeholder_text(Some(locale.filter_text_placeholder()));
    }
}
