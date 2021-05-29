mod clear_cache_dialog;
mod config_dialog;
mod configuration;
mod connect_config_dialog;
mod connect_detail_buttons;
mod connect_menu_options;
mod connect_output_dialog;
mod connect_search_event;
mod connect_selection_change;
mod detail_pane;
mod download_progress;
mod filter_bar;
mod game_player;
mod gui_data;
mod image_loader;
mod initializable;
mod initialize_gui;
mod installer;
mod list_view;
mod locales;
mod main_menu;
mod output_dialog;
mod progress_dialog;
mod quake_file;
mod request_utils;

use dirs::config_dir;
use flexi_logger::{Age, Cleanup, Criterion, LevelFilter, LogSpecBuilder, Logger, Naming};
use initialize_gui::initialize_gui;
use log::*;
use std::path::PathBuf;

fn main() {
    let mut log_builder = LogSpecBuilder::new();
    log_builder.default(LevelFilter::Debug);
    log_builder.module("reqwest", LevelFilter::Debug);
    log_builder.module("mio", LevelFilter::Warn); // used by reqwest
    log_builder.module("want", LevelFilter::Warn); // used by reqwest
    let mut log_dir = config_dir().expect("No config dir found");
    log_dir.push("QInjector");
    log_dir.push("logs");

    Logger::with(log_builder.build())
        .duplicate_to_stdout(flexi_logger::Duplicate::All)
        .log_to_file()
        .format(flexi_logger::colored_with_thread)
        .o_directory(Some(log_dir))
        .rotate(
            Criterion::Age(Age::Day),
            Naming::Numbers,
            Cleanup::KeepLogFiles(5),
        )
        .set_palette("196;208;-;7;10".to_string())
        .start()
        .unwrap_or_else(|e| panic!("Logger initialization failed with {}", e));
    gtk::init().expect("Failed to init gtk");
    initialize_application();
    gtk::main();
}

fn initialize_application() {
    trace!("Starting application");
    let mut base_config_dir = config_dir().expect("No config dir found");
    base_config_dir.push("QInjector");
    let base_needed = match std::fs::create_dir(&base_config_dir) {
        Ok(_) => {
            trace!("Made base config directory");
            true
        }
        Err(_) => {
            trace!("Base config directory exists");
            false
        }
    };
    let images_needed = init_dir_by_name(&mut base_config_dir, "images");
    let logs_needed = init_dir_by_name(&mut base_config_dir, "logs");

    // if we needed to remake directories pop the init modal
    initialize_gui(base_needed || images_needed || logs_needed);
}

fn init_dir_by_name(config_dir: &mut PathBuf, name: &str) -> bool {
    config_dir.push(name);
    match std::fs::create_dir(config_dir) {
        Ok(_) => {
            trace!("Made {} directory", name);
            true
        }
        Err(_) => {
            trace!("{} directory exists", name);
            false
        }
    }
}
