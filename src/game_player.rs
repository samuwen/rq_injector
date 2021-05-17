use derive_builder::Builder;
use getset::Getters;
use log::*;
use std::process::{Command, Output};

#[derive(Builder, Clone, Debug, Getters)]
pub struct GamePlayer {
    quake_exe: String,
    quake_dir: String,
    map_id: String,
    start_map: String,
    command_line: Option<String>,
}

impl GamePlayer {
    pub fn play_quake_map(&self) -> Output {
        info!("Attempting to play game: {}", self.map_id);
        debug!("start map: {}", self.start_map);
        let mut cmd = Command::new(self.quake_exe.to_owned());
        cmd.arg("-basedir").arg(self.quake_dir.to_owned());
        if let Some(line) = self.command_line.clone() {
            debug!("Adding command line: {}", line);
            line.split(" ").for_each(|sp| {
                cmd.arg(sp);
            });
        }
        cmd.arg("+map").arg(self.start_map.to_owned());
        cmd.output().expect("Couldn't get output")
    }
}
