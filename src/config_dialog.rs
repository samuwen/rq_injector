use crate::app::QInjector;
use gtk::prelude::*;
use gtk::{Builder, Button, CheckButton, Dialog, Entry};
use log::*;

#[derive(Clone)]
pub struct ConfigDialog {
    pub dlg_config: Dialog,

    pub btn_ok: Button,
    pub btn_cancel: Button,
    pub btn_quake_dir: Button,
    pub btn_quake_exe: Button,
    pub btn_download_dir: Button,

    pub ent_command_opts: Entry,
    pub ent_quake_dir: Entry,
    pub ent_quake_exe: Entry,
    pub ent_download_dir: Entry,

    pub chk_rogue: CheckButton,
    pub chk_hipnotic: CheckButton,
}

impl ConfigDialog {
    pub fn create_from_builder(builder: &Builder) -> Self {
        let dlg_config: Dialog = builder
            .get_object("dlg_config")
            .expect("Failed to get dlg_config");
        let btn_ok: Button = builder
            .get_object("dlg_config_btn_ok")
            .expect("Failed to get btn_ok");
        let btn_cancel: Button = builder
            .get_object("dlg_config_btn_cancel")
            .expect("Failed to get btn_cancel");
        let btn_quake_exe: Button = builder
            .get_object("dlg_quake_exe_btn_select")
            .expect("Failed to get btn_quake_exe");
        let btn_download_dir: Button = builder
            .get_object("dlg_download_dir_btn_select")
            .expect("Failed to get btn_download_dir");
        let btn_quake_dir: Button = builder
            .get_object("dlg_quake_dir_btn_select")
            .expect("Failed to get btn_quake_dir");
        let ent_command_opts: Entry = builder
            .get_object("ent_command_opts")
            .expect("Failed to get ent_command_opts");
        let ent_quake_dir: Entry = builder
            .get_object("ent_quake_dir")
            .expect("Failed to get ent_quake_dir");
        let ent_quake_exe: Entry = builder
            .get_object("ent_quake_exe")
            .expect("Failed to get ent_quake_exe");
        let ent_download_dir: Entry = builder
            .get_object("ent_download_dir")
            .expect("Failed to get ent_download_dir");
        let chk_rogue: CheckButton = builder
            .get_object("dlg_btn_rogue")
            .expect("Failed to get chk_rogue");
        let chk_hipnotic: CheckButton = builder
            .get_object("dlg_btn_hipnotic")
            .expect("Failed to get chk_hipnotic");
        Self {
            dlg_config,
            btn_ok,
            btn_cancel,
            btn_quake_dir,
            btn_quake_exe,
            btn_download_dir,
            ent_command_opts,
            ent_quake_dir,
            ent_quake_exe,
            ent_download_dir,
            chk_rogue,
            chk_hipnotic,
        }
    }
}
