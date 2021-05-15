use gtk::prelude::*;
use gtk::{Builder, Button, Image, Label, ScrolledWindow};

#[derive(Clone)]
pub struct DetailPane {
    pub lbl_title: Label,
    pub img_current_map: Image,
    pub lbl_description: Label,
    pub btn_install: Button,
    pub btn_uninstall: Button,
    pub btn_play: Button,
    pub sw_details: ScrolledWindow,
}

impl DetailPane {
    pub fn create_from_builder(builder: &Builder) -> Self {
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
        let sw_details: ScrolledWindow = builder
            .get_object("sw_details")
            .expect("Failed to get sw_detail");
        Self {
            lbl_title,
            lbl_description,
            img_current_map,
            btn_play,
            btn_install,
            btn_uninstall,
            sw_details,
        }
    }
}
