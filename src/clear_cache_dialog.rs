use gtk::prelude::*;
use gtk::{Builder, Button, Dialog, TextView};

#[derive(Clone)]
pub struct ClearCacheDialog {
    pub dlg_you_sure: Dialog,
    pub btn_confirm_clear_cache: Button,
    pub btn_cancel_clear_cache: Button,
    pub txt_clear_cache_warning: TextView,
}

impl ClearCacheDialog {
    pub fn create_from_builder(builder: &Builder) -> Self {
        let dlg_you_sure: Dialog = builder
            .get_object("dlg_you_sure")
            .expect("Failed to get dlg_you_sure");
        let btn_confirm_clear_cache: Button = builder
            .get_object("btn_confirm_clear_cache")
            .expect("Failed to get btn_confirm_clear_cache");
        let btn_cancel_clear_cache: Button = builder
            .get_object("btn_cancel_clear_cache")
            .expect("Failed to get btn_cancel_clear_cache");
        let txt_clear_cache_warning: TextView = builder
            .get_object("txt_clear_cache_warning")
            .expect("Failed to get txt_clear_cache_warning");
        Self {
            dlg_you_sure,
            btn_confirm_clear_cache,
            btn_cancel_clear_cache,
            txt_clear_cache_warning,
        }
    }
}
