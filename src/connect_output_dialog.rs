use crate::gui_data::GuiData;
use gtk::prelude::*;
use log::*;

pub fn connect_ok(gui_data: &GuiData) {
    trace!("Initializing output ok button");
    let dialog = gui_data.output_dialog.dlg_output.clone();
    let button = gui_data.output_dialog.btn_ok.clone();
    let text_output = gui_data.output_dialog.txt_output.clone();
    button.connect_clicked(move |_| {
        trace!("Deleting text and hiding output dialog");
        text_output.get_buffer().unwrap().set_text("");
        dialog.hide();
    });
}
