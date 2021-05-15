use glib::types::Type;
use gtk::prelude::*;
use gtk::{Builder, ListStore, ScrolledWindow, TreeView};

#[derive(Clone)]
pub struct ListView {
    pub sw_list: ScrolledWindow,
    pub list_store: ListStore,
    pub tree_view: TreeView,
}

impl ListView {
    pub fn create_from_builder(builder: &Builder) -> Self {
        let col_types: [Type; 6] = [
            Type::Bool,
            Type::String,
            Type::String,
            Type::String,
            Type::String,
            Type::String,
        ];
        let sw_list: ScrolledWindow = builder
            .get_object("sw_list")
            .expect("Failed to get sw_list");
        let list_store = gtk::ListStore::new(&col_types);
        let tree_view = gtk::TreeView::with_model(&list_store);
        Self {
            sw_list,
            list_store,
            tree_view,
        }
    }
}
