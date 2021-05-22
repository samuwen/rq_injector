use crate::initializable::Initializable;
use crate::locales::Locale;
use gtk::prelude::*;
use gtk::{Builder, Button, CheckButton, Dialog, Entry, Label};

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

    pub lbl_config_dialog_header: Label,
    pub lbl_config_dialog_command_line: Label,
    pub lbl_config_dialog_quake_dir: Label,
    pub lbl_config_dialog_quake_exe: Label,
    pub lbl_config_dialog_download_dir: Label,
    pub lbl_config_dialog_exp_packs: Label,

    pub chk_rogue: CheckButton,
    pub chk_hipnotic: CheckButton,
}

impl ConfigDialog {
    pub fn create_from_builder(builder: &Builder) -> Self {
        let dlg_config: Dialog = builder
            .get_object("dlg_config")
            .expect("Failed to get dlg_config");
        dlg_config.connect_delete_event(move |me, _| me.hide_on_delete());
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
        let lbl_config_dialog_header: Label = builder
            .get_object("lbl_config_dialog_header")
            .expect("Failed to get lbl_config_dialog_header");
        let lbl_config_dialog_quake_dir: Label = builder
            .get_object("lbl_config_dialog_quake_dir")
            .expect("Failed to get lbl_config_dialog_quake_dir");
        let lbl_config_dialog_quake_exe: Label = builder
            .get_object("lbl_config_dialog_quake_exe")
            .expect("Failed to get lbl_config_dialog_quake_exe");
        let lbl_config_dialog_download_dir: Label = builder
            .get_object("lbl_config_dialog_download_dir")
            .expect("Failed to get lbl_config_dialog_download_dir");
        let lbl_config_dialog_command_line: Label = builder
            .get_object("lbl_config_dialog_command_line")
            .expect("Failed to get lbl_config_dialog_command_line");
        let lbl_config_dialog_exp_packs: Label = builder
            .get_object("lbl_config_dialog_exp_packs")
            .expect("Failed to get lbl_config_dialog_exp_packs");
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
            lbl_config_dialog_header,
            lbl_config_dialog_quake_dir,
            lbl_config_dialog_quake_exe,
            lbl_config_dialog_command_line,
            lbl_config_dialog_download_dir,
            lbl_config_dialog_exp_packs,
            chk_rogue,
            chk_hipnotic,
        }
    }
}

impl Initializable for ConfigDialog {
    fn init_text(&self, locale: &Locale) {
        self.dlg_config.set_title(locale.config_dialog_title());
        self.btn_ok.set_label(locale.universal_ok_button());
        self.btn_cancel.set_label(locale.universal_cancel_button());
        self.btn_download_dir
            .set_label(locale.config_dialog_select_button_text());
        self.btn_quake_dir
            .set_label(locale.config_dialog_select_button_text());
        self.btn_quake_exe
            .set_label(locale.config_dialog_select_button_text());
        self.lbl_config_dialog_header
            .set_label(locale.config_dialog_header_text());
        self.lbl_config_dialog_command_line
            .set_label(locale.config_dialog_command_line_text());
        self.lbl_config_dialog_quake_dir
            .set_label(locale.config_dialog_quake_dir_text());
        self.lbl_config_dialog_quake_exe
            .set_label(locale.config_dialog_quake_exe_text());
        self.lbl_config_dialog_download_dir
            .set_label(locale.config_dialog_download_dir_text());
        self.lbl_config_dialog_exp_packs
            .set_label(locale.config_dialog_expansion_pack_text());
        self.chk_hipnotic
            .set_label(locale.config_dialog_expansion_hip_text());
        self.chk_rogue
            .set_label(locale.config_dialog_expansion_rogue_text());
    }
}
