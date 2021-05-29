use crate::connect_config_dialog;
use crate::connect_detail_buttons;
use crate::connect_menu_options;
use crate::connect_output_dialog;
use crate::connect_search_event;
use crate::connect_selection_change;
use crate::gui_data::GuiData;
use crate::initializable::Initializable;
use crate::list_view::populate_list_view;

pub fn initialize_gui(first_time_init: bool) {
    let gui_data = GuiData::new();

    initialize_menu(&gui_data);
    initialize_config_dialog(&gui_data);
    initialize_detail_buttons(&gui_data);
    initialize_output_dialog(&gui_data);
    initialize_filter_bar(&gui_data);
    let tree_view = gui_data.list_view.tree_view.clone();
    connect_selection_change::connect_selection_change(&gui_data, &tree_view);
    create_list_view(&gui_data, first_time_init);
}

fn create_list_view(gui_data: &GuiData, first_time_init: bool) {
    let list_view = gui_data.list_view.clone();

    list_view.initialize(gui_data);
    if first_time_init {
        // show config dialog if its first time init, so the user sets their
        // data while we grab the database
        log::debug!("First time initialization");
        let dialog = gui_data.config_dialog.clone();
        let config_state = gui_data.shared_config_state.clone();
        dialog.show(config_state);
    }
    populate_list_view(gui_data);
    connect_search_event::connect_search_event(&gui_data);
}

fn initialize_config_dialog(gui_data: &GuiData) {
    let config_dialog = gui_data.config_dialog.clone();
    init_text(config_dialog, gui_data);
    connect_config_dialog::connect_activate(gui_data);
    connect_config_dialog::connect_cancel(gui_data);
    connect_config_dialog::connect_ok(gui_data);
    connect_config_dialog::connect_selects(gui_data);
}

fn initialize_detail_buttons(gui_data: &GuiData) {
    let detail_pane = gui_data.detail_pane.clone();
    init_text(detail_pane, gui_data);
    connect_detail_buttons::connect_install_map(gui_data);
    connect_detail_buttons::connect_uninstall_map(gui_data);
    connect_detail_buttons::connect_play_button(gui_data);
}

fn initialize_output_dialog(gui_data: &GuiData) {
    let output_dialog = gui_data.output_dialog.clone();
    init_text(output_dialog, gui_data);
    connect_output_dialog::connect_ok(gui_data);
}

fn initialize_menu(gui_data: &GuiData) {
    let config_state = gui_data.shared_config_state.clone();
    let main_menu = gui_data.main_menu.clone();
    main_menu.init_states(*config_state.borrow().is_offline());
    init_text(main_menu, gui_data);
    let clear_cache_dialog = gui_data.clear_cache_dialog.clone();
    init_text(clear_cache_dialog, gui_data);

    connect_menu_options::connect_menu_quit(gui_data);
    connect_menu_options::connect_close(gui_data);
    connect_menu_options::connect_reload(gui_data);
    connect_menu_options::connect_offline(gui_data);
    connect_menu_options::connect_clear_cache_ok(gui_data);
    connect_menu_options::connect_clear_cache_cancel(gui_data);
    connect_menu_options::connect_clear_cache(gui_data);
}

fn initialize_filter_bar(gui_data: &GuiData) {
    let filter_bar = gui_data.filter_bar.clone();
    init_text(filter_bar, gui_data);
    connect_detail_buttons::connect_install_random_map(gui_data);
}

fn init_text(element: impl Initializable, gui_data: &GuiData) {
    let config_state = gui_data.shared_config_state.clone();
    element.init_text(config_state.borrow().current_locale());
}
