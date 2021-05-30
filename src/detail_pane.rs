use crate::initializable::Initializable;
use crate::locales::Locale;
use crate::quake_file::QuakeFile;
use chrono::NaiveDate;
use gdk_pixbuf::{Pixbuf, PixbufAnimation};
use gtk::prelude::*;
use gtk::{Builder, Button, ComboBoxText, Image, Label, ProgressBar, ScrolledWindow};
use log::*;
use std::cell::RefCell;
use std::rc::Rc;

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
    pub box_progress: gtk::Box,
    pub dropdown: ComboBoxText,
    progress_bars: Rc<RefCell<Vec<ProgressBar>>>,
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
        let box_progress: gtk::Box = builder
            .get_object("box_progress")
            .expect("Failed to get box_progress");
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
            box_progress,
            dropdown,
            progress_bars: Rc::new(RefCell::new(vec![])),
        }
    }

    pub fn update(&self, file: &QuakeFile, is_local: bool, is_offline: bool, date_format: String) {
        trace!("Updating detail view");
        self.lbl_title.set_text(file.title());
        self.lbl_description.set_text(file.description());
        let naive_date = NaiveDate::parse_from_str(file.date(), "%d.%m.%Y").unwrap();
        self.lbl_date
            .set_text(&naive_date.format(&date_format).to_string());
        let size_text = convert_size_string(file.size());
        self.lbl_size.set_text(&size_text);
        self.btn_install.set_sensitive(!is_local && !is_offline);
        self.btn_uninstall.set_sensitive(is_local);
        self.btn_play.set_sensitive(is_local);

        self.img_current_map.set_visible(false); // hide image until it loads
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
        let mut path = std::path::PathBuf::new();
        path.push("resources");
        path.push("loading.gif");
        let pixbuf = PixbufAnimation::from_file(path).unwrap();
        self.set_spinner(pixbuf);
    }

    pub fn clear(&self) {
        self.lbl_title.set_text("");
        self.lbl_description.set_text("");
        self.lbl_date.set_text("");
        self.lbl_size.set_text("");
        self.btn_install.set_sensitive(false);
        self.btn_uninstall.set_sensitive(false);
        self.btn_play.set_sensitive(false);
        self.img_current_map.set_visible(false);
        self.dropdown.remove_all();
    }

    pub fn set_spinner(&self, anim: PixbufAnimation) {
        self.img_current_map.set_from_animation(&anim);
        self.img_current_map.set_visible(true);
    }

    pub fn update_image(&self, pixbuf: Pixbuf) {
        self.img_current_map.set_from_pixbuf(Some(&pixbuf));
        self.img_current_map.set_visible(true);
    }

    pub fn add_progress_bar(&mut self, map_id: &String) {
        let progress_bar = ProgressBar::new();
        progress_bar.set_text(Some(map_id));
        progress_bar.set_show_text(true);
        self.box_progress.add(&progress_bar);
        self.progress_bars.borrow_mut().push(progress_bar);
        self.box_progress.show_all();
    }

    pub fn update_progress_bar(&self, map_id: &String, fraction: f64) {
        let bars = self.progress_bars.borrow();
        let bar = bars
            .iter()
            .find(|bar| map_id.contains(bar.get_text().unwrap().as_str()))
            .unwrap();
        bar.set_fraction(fraction);
    }
}

impl Initializable for DetailPane {
    fn init_text(&self, locale: &Locale) {
        self.btn_install.set_label(locale.install_button_text());
        self.btn_uninstall.set_label(locale.uninstall_button_text());
        self.btn_play.set_label(locale.play_button_text());
    }
}

fn convert_size_string(size_string: &String) -> String {
    let size_int = u32::from_str_radix(size_string, 10).unwrap();
    let decimal = size_int as f64 / 1000.0;
    trace!("Converted {} to {}", size_string, decimal);
    format!("{} mb", decimal)
}
