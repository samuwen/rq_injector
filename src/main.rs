mod config_dialog;
mod configuration;
mod connect_config_dialog;
mod connect_detail_buttons;
mod connect_output_dialog;
mod connect_quit;
mod connect_search_event;
mod connect_selection_change;
mod detail_pane;
mod filter_bar;
mod game_player;
mod gui_data;
mod image_loader;
mod initialize_gui;
mod installer;
mod list_view;
mod main_menu;
mod output_dialog;
mod quake_file;

use dirs::config_dir;
use flexi_logger::{LevelFilter, LogSpecBuilder, Logger};
use initialize_gui::initialize_gui;
use log::*;
use std::path::PathBuf;

fn main() {
    let mut log_builder = LogSpecBuilder::new();
    log_builder.default(LevelFilter::Trace);
    log_builder.module("reqwest", LevelFilter::Debug);
    log_builder.module("mio", LevelFilter::Warn); // used by reqwest
    log_builder.module("want", LevelFilter::Warn); // used by reqwest

    Logger::with(log_builder.build())
        .duplicate_to_stdout(flexi_logger::Duplicate::All)
        .format(flexi_logger::colored_with_thread)
        .set_palette("196;208;-;7;10".to_string())
        .start()
        .unwrap_or_else(|e| panic!("Logger initialization failed with {}", e));
    gtk::init().expect("Failed to init gtk");
    initialize_application();
    gtk::main();
}

fn initialize_application() {
    trace!("Starting application");
    let mut base_config_dir = config_dir().expect("No home dir found");
    base_config_dir.push("QInjector");
    match std::fs::create_dir(&base_config_dir) {
        Ok(_) => trace!("Made base config directory"),
        Err(_) => trace!("Base config directory exists"),
    }
    init_images_dir(base_config_dir);

    initialize_gui();
}

fn init_images_dir(mut config_dir: PathBuf) {
    config_dir.push("images");
    match std::fs::create_dir(config_dir) {
        Ok(_) => trace!("Made images directory"),
        Err(_) => trace!("images directory exists"),
    }
}
