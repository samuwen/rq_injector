use crate::quake_file::QuakeFile;
use gdk_pixbuf::Pixbuf;
use gtk::prelude::*;
use gtk::{Builder, Button, ComboBoxText, Image, Label, ScrolledWindow};
use log::*;

#[derive(Clone)]
pub struct DetailPane {
    pub lbl_title: Label,
    pub img_current_map: Image,
    pub lbl_description: Label,
    pub btn_install: Button,
    pub btn_uninstall: Button,
    pub btn_play: Button,
    pub lbl_date: Label,
    pub lbl_size: Label,
    pub sw_details: ScrolledWindow,
    pub dropdown: ComboBoxText,
}

impl DetailPane {
    pub fn create_from_builder(builder: &Builder) -> Self {
        trace!("Initializing detail pane");
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
        let lbl_date: Label = builder
            .get_object("lbl_date")
            .expect("Failed to get lbl_date");
        let lbl_size: Label = builder
            .get_object("lbl_size")
            .expect("Failed to get lbl_size");
        let dropdown: ComboBoxText = builder
            .get_object("start_combo_box")
            .expect("Failed to get start_combo_box");
        Self {
            lbl_title,
            lbl_description,
            img_current_map,
            btn_play,
            btn_install,
            btn_uninstall,
            sw_details,
            lbl_date,
            lbl_size,
            dropdown,
        }
    }

    pub fn update(&self, file: &QuakeFile, pixbuf: Pixbuf) {
        trace!("Updating detail view");
        self.lbl_title.set_text(file.title());
        self.lbl_description.set_text(file.description());
        self.lbl_date.set_text(file.date());
        let size_text = convert_size_string(file.size());
        self.lbl_size.set_text(&size_text);
        let is_local = *file.installed_locally();
        self.btn_install.set_sensitive(!is_local);
        self.btn_uninstall.set_sensitive(is_local);
        self.btn_play.set_sensitive(is_local);

        self.img_current_map.set_from_pixbuf(Some(&pixbuf));
        self.img_current_map.set_visible(true);
        let start_maps = file.tech_info().start_map();
        self.dropdown.remove_all();
        if start_maps.len() == 0 {
            self.dropdown.append_text("start");
        } else {
            for map in start_maps {
                self.dropdown.append_text(map);
            }
            let model = self.dropdown.get_model().unwrap();
            let iter = model.get_iter_first().expect("nothing in dropdown");
            self.dropdown.set_active_iter(Some(&iter));
        }
        self.dropdown.set_sensitive(start_maps.len() > 2);
    }
}

fn convert_size_string(size_string: &String) -> String {
    let size_int = u32::from_str_radix(size_string, 10).unwrap();
    let decimal = size_int as f64 / 1000.0;
    trace!("Converted {} to {}", size_string, decimal);
    format!("{} mb", decimal)
}
