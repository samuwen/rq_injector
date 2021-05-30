use crate::configuration::Configuration;
use crate::initializable::Initializable;
use crate::locales::Locale;
use gtk::prelude::*;
use gtk::{Builder, ComboBoxText, Label};
use std::cell::RefCell;
use std::fs::read_dir;
use std::rc::Rc;

#[derive(Clone)]
pub struct PersonalConfigTab {
    pub lbl_personalize_title: Label,
    pub lbl_config_personal_language: Label,
    pub lbl_config_personal_dateformat: Label,
    pub dropdown_choose_language: ComboBoxText,
    pub dropdown_choose_dateformat: ComboBoxText,
}

impl PersonalConfigTab {
    pub fn create_from_builder(builder: &Builder) -> Self {
        let lbl_personalize_title: Label = builder
            .get_object("lbl_personalize_title")
            .expect("Failed to get lbl_personalize_title");
        let lbl_config_personal_language: Label = builder
            .get_object("lbl_config_personal_language")
            .expect("Failed to get lbl_config_personal_language");
        let lbl_config_personal_dateformat: Label = builder
            .get_object("lbl_config_personal_dateformat")
            .expect("Failed to get lbl_config_personal_dateformat");
        let dropdown_choose_language: ComboBoxText = builder
            .get_object("dropdown_choose_language")
            .expect("Failed to get dropdown_choose_language");
        let dropdown_choose_dateformat: ComboBoxText = builder
            .get_object("dropdown_choose_dateformat")
            .expect("Failed to get dropdown_choose_dateformat");
        Self {
            lbl_personalize_title,
            lbl_config_personal_language,
            lbl_config_personal_dateformat,
            dropdown_choose_language,
            dropdown_choose_dateformat,
        }
    }

    pub fn show(&self, shared_config: Rc<RefCell<Configuration>>) {
        let borrow = shared_config.borrow();
        self.dropdown_choose_language.remove_all();
        self.dropdown_choose_dateformat.remove_all();
        read_dir(borrow.locale_resources_dir())
            .expect("Couldn't read locale dir")
            .for_each(|entry| {
                let locale_title = entry
                    .unwrap()
                    .file_name()
                    .into_string()
                    .expect("String is not unicode")
                    .replace(".xml", "");
                self.dropdown_choose_language.append_text(&locale_title);
            });
        let locale_name = borrow.current_locale_choice().get_choice();
        set_active(self.dropdown_choose_language.clone(), &locale_name);
        let formats = vec!["mm-dd-yyyy", "mm.dd.yyyy", "dd-mm-yyyy", "dd.mm.yyyy"];
        formats
            .iter()
            .for_each(|format| self.dropdown_choose_dateformat.append_text(format));
        set_active(
            self.dropdown_choose_dateformat.clone(),
            borrow.date_format(),
        );
    }
}

impl Initializable for PersonalConfigTab {
    fn init_text(&self, locale: &Locale) {
        self.lbl_personalize_title
            .set_label(locale.config_dialog_personal_title());
        self.lbl_config_personal_language
            .set_label(locale.config_dialog_language_selector());
        self.lbl_config_personal_dateformat
            .set_label(locale.config_dialog_dateformat_selector());
    }
}

fn set_active(dropdown: ComboBoxText, val_to_match: &String) {
    let model = dropdown.get_model().unwrap();
    model.foreach(|model, _path, iter| {
        let string_res: Result<Option<String>, glib::value::GetError> =
            model.get_value(&iter, 0).get();
        let text = string_res.unwrap().unwrap();
        let found = &text == val_to_match;
        if found {
            dropdown.set_active_iter(Some(&iter));
        }
        found
    });
}
