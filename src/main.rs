mod app;
mod config_dialog;
mod configuration;
mod connect_config_dialog;
mod connect_detail_buttons;
mod connect_output_dialog;
mod connect_quit;
mod detail_pane;
mod filter_bar;
mod gui_data;
mod initialize_gui;
mod installer;
mod list_view;
mod main_menu;
mod output_dialog;
mod quake_file;

use app::initialize_application;
use flexi_logger::{LevelFilter, LogSpecBuilder, Logger};

fn main() {
    let mut log_builder = LogSpecBuilder::new();
    log_builder.default(LevelFilter::Trace);
    log_builder.module("reqwest", LevelFilter::Debug);
    log_builder.module("mio", LevelFilter::Warn); // used by reqwest
    log_builder.module("want", LevelFilter::Warn); // used by reqwest

    Logger::with(log_builder.build())
        .duplicate_to_stdout(flexi_logger::Duplicate::All)
        .format(flexi_logger::colored_default_format)
        .set_palette("196;208;-;7;10".to_string())
        .start()
        .unwrap_or_else(|e| panic!("Logger initialization failed with {}", e));
    gtk::init().expect("Failed to init gtk");
    initialize_application();
    gtk::main();
}
