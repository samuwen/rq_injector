use crate::configuration::Configuration;
use crate::engine_config_tab::EngineConfigTab;
use crate::initializable::Initializable;
use crate::locales::Locale;
use crate::personal_config_tab::PersonalConfigTab;
use gtk::prelude::*;
use gtk::{Builder, Button, Dialog};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct ConfigDialog {
    pub dlg_config: Dialog,

    pub btn_ok: Button,
    pub btn_cancel: Button,
    pub engine_config_tab: EngineConfigTab,
    pub personal_config_tab: PersonalConfigTab,
}

impl ConfigDialog {
    pub fn create_from_builder(builder: &Builder) -> Self {
        let dlg_config: Dialog = builder
            .get_object("dlg_config")
            .expect("Failed to get dlg_config");
        dlg_config.connect_delete_event(move |me, _| me.hide_on_delete());
        let btn_ok: Button = builder
            .get_object("dlg_config_btn_ok")
            .expect("Failed to get btn_ok");
        let btn_cancel: Button = builder
            .get_object("dlg_config_btn_cancel")
            .expect("Failed to get btn_cancel");
        let engine_config_tab = EngineConfigTab::create_from_builder(builder);
        let personal_config_tab = PersonalConfigTab::create_from_builder(builder);
        Self {
            dlg_config,
            btn_ok,
            btn_cancel,
            engine_config_tab,
            personal_config_tab,
        }
    }

    pub fn show(&self, shared_config: Rc<RefCell<Configuration>>) {
        self.engine_config_tab.show(shared_config.clone());
        self.personal_config_tab.show(shared_config);
        self.dlg_config.show_all();
    }
}

impl Initializable for ConfigDialog {
    fn init_text(&self, locale: &Locale) {
        self.dlg_config.set_title(locale.config_dialog_title());
        self.btn_ok.set_label(locale.universal_ok_button());
        self.btn_cancel.set_label(locale.universal_cancel_button());
        self.engine_config_tab.init_text(locale);
        self.personal_config_tab.init_text(locale);
    }
}
