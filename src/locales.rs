use getset::Getters;
use log::*;
use quick_xml::de::{from_reader, DeError};
use serde::{Deserialize, Serialize, Serializer};
use std::fmt::Display;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum LocaleChoice {
    EnUs,
    EsEs,
}

// This crazy approach is because quick_xml doesn't really deal with untagged enums very well. As
// a result it ends up serializing the variant as a tag (ie: <EnUS />), making it useless for
// retrieving the info and handling localization loading at runtime. It didn't feel like it made a
// lot of sense to store the locale data in the config file so we have to hack around it like this.
#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum Nester {
    NestedEnum(LocaleChoice), // used for serialization
    Other(String),            // what we get back when we deserialize
}

impl Nester {
    pub fn get_choice(&self) -> String {
        match self {
            // this should never get deserialized
            Nester::NestedEnum(_) => panic!("you shouldn't have come here"),
            // we serialize to this, so this gets us the string value
            Nester::Other(s) => s.to_owned(),
        }
    }
}

// serde serializer
impl Serialize for Nester {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NestedEnum(ref b) => serializer.serialize_str(match b {
                LocaleChoice::EnUs => "en-US",
                LocaleChoice::EsEs => "es-ES",
            }),
            Self::Other(s) => serializer.serialize_str(s),
        }
    }
}

// a helper. Probably could abstract this to a suite of constants to ensure we don't make typos.
impl LocaleChoice {
    pub fn get_name(&self) -> String {
        let val = match self {
            LocaleChoice::EnUs => "en-US",
            LocaleChoice::EsEs => "es-ES",
        };
        val.to_string()
    }
}

#[derive(Clone, Debug, Default, Deserialize, Getters)]
#[getset(get = "pub")]
pub struct Locale {
    app_title: String,
    file_menu_name: String,
    reload_database_menu_option: String,
    check_installed_menu_option: String,
    clear_cache_menu_option: String,
    offline_menu_option: String,
    quit_menu_option: String,
    configuration_menu_name: String,
    configuration_menu_option: String,
    install_random_map_button: String,
    clear_filter_text_button: String,
    filter_text_placeholder: String,
    id_column_name: String,
    title_column_name: String,
    author_column_name: String,
    released_column_name: String,
    rating_column_name: String,
    install_button_text: String,
    uninstall_button_text: String,
    play_button_text: String,
    config_dialog_title: String,
    config_dialog_header_text: String,
    config_dialog_command_line_text: String,
    config_dialog_quake_dir_text: String,
    config_dialog_quake_exe_text: String,
    config_dialog_download_dir_text: String,
    config_dialog_expansion_pack_text: String,
    config_dialog_expansion_hip_text: String,
    config_dialog_expansion_rogue_text: String,
    config_dialog_select_button_text: String,
    universal_ok_button: String,
    universal_cancel_button: String,
    clear_dialog_title: String,
    clear_dialog_text: String,
    output_dialog_title: String,
    database_corrupt_error: String,
}

pub fn init_locale(locale_dir: PathBuf, locale_choice: String) -> Locale {
    trace!("Initializing locales");
    let file_result = read_locale_file(locale_dir.clone(), &locale_choice);
    match file_result {
        Ok(f) => {
            info!("Locale file {} loaded successfully", locale_choice);
            f
        }
        Err(e) => {
            error!(
                "Couldn't parse {} locale file. Defaulting to English\nError: {}",
                locale_choice, e
            );
            let file_result = read_locale_file(locale_dir, "en-us");
            match file_result {
                Ok(f) => f,
                Err(e) => {
                    error!("No english localization file found. You stink.");
                    panic!("{}", e);
                }
            }
        }
    }
}

fn read_locale_file<S: AsRef<str> + Display>(dir: PathBuf, name: S) -> Result<Locale, DeError> {
    let mut file_path = dir.clone();
    let file_name = format!("{}.xml", name);
    file_path.push(file_name);
    let file = File::open(&file_path).expect(&format!("No locale file keyed by name: {}", name));
    let reader = BufReader::new(file);
    from_reader(reader)
}
