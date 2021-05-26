use gtk::prelude::*;
use gtk::{Builder, Dialog, Label};
use log::*;

#[derive(Clone)]
pub struct ProgressDialog {
    pub dlg_db_progress: Dialog,
    pub lbl_db_progress: Label,
}

impl ProgressDialog {
    pub fn create_from_builder(builder: &Builder) -> Self {
        trace!("Initializing progress dialog");
        let lbl_db_progress: Label = builder
            .get_object("lbl_db_progress")
            .expect("Failed to get lbl_db_progress");
        let dlg_db_progress: Dialog = builder
            .get_object("dlg_db_progress")
            .expect("Failed to get dlg_db_progress");
        Self {
            dlg_db_progress,
            lbl_db_progress,
        }
    }
}
