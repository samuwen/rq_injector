use dirs::config_dir;
use std::path::PathBuf;

pub fn get_config_path() -> PathBuf {
    let mut path = config_dir().expect("No config dir found");
    path.push("QInjector");
    path
}
