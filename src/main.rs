mod configuration;
mod connect_quit;
mod gui_data;
mod initialize_gui;
mod quake_file;

use configuration::{Configuration, LocalMaps};
use connect_quit::connect_menu_quit;
use flexi_logger::{LevelFilter, LogSpecBuilder, Logger};
use gui_data::GuiData;
use initialize_gui::initialize_gui;
use quake_file::{initialize_data, QuakeFile};

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
    let data = initialize_data();
    gtk::init().expect("Failed to init gtk");
    let gui_data = GuiData::new();
    initialize_gui(&gui_data, &data);
    gtk::main();
}
