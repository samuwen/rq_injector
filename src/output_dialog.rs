use crate::initializable::Initializable;
use crate::locales::Locale;
use gtk::prelude::*;
use gtk::{Builder, Button, Dialog, TextView};

#[derive(Clone)]
pub struct OutputDialog {
    pub dlg_output: Dialog,
    pub btn_ok: Button,
    pub txt_output: TextView,
}

impl OutputDialog {
    pub fn create_from_builder(builder: &Builder) -> Self {
        let dlg_output: Dialog = builder
            .get_object("dlg_output")
            .expect("Failed to get dlg_output");
        let btn_ok: Button = builder
            .get_object("btn_confirm_game_output")
            .expect("Failed to get btn_ok");
        let txt_output: TextView = builder
            .get_object("txt_output")
            .expect("Failed to get txt_output");
        Self {
            dlg_output,
            btn_ok,
            txt_output,
        }
    }
}

impl Initializable for OutputDialog {
    fn init_text(&self, locale: &Locale) {
        self.btn_ok.set_label(locale.universal_ok_button());
        self.dlg_output.set_title(locale.output_dialog_title());
    }
}
