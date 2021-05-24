use derive_builder::Builder;
use getset::Getters;
use log::*;
use std::process::{Command, Output};

#[derive(Builder, Clone, Debug, Getters)]
pub struct GamePlayer {
    quake_exe: String,
    quake_dir: String,
    map_id: String,
    start_map: Option<String>,
    command_line: Option<String>,
}

impl GamePlayer {
    pub fn play_quake_map(&self) -> Output {
        info!("Attempting to play game: {}", self.map_id);
        let mut command_as_string = format!("{} -basedir {}", self.quake_exe, self.quake_dir);
        let mut cmd = Command::new(self.quake_exe.to_owned());
        cmd.arg("-basedir").arg(self.quake_dir.to_owned());
        if let Some(line) = self.command_line.clone() {
            debug!("Adding command line: {}", line);
            line.split(" ").for_each(|sp| {
                command_as_string.push_str(sp);
                command_as_string.push_str(" ");
                cmd.arg(sp);
            });
        } else {
            cmd.arg("-game").arg(&self.map_id);
            command_as_string.push_str("-game");
            command_as_string.push_str(&self.map_id);
        }
        if let Some(start_map) = self.start_map.clone() {
            cmd.arg("+map").arg(start_map.to_owned());
            command_as_string.push_str(&format!(" +map {}", start_map));
        }
        debug!("{}", command_as_string);
        cmd.output().expect("Couldn't get output")
    }
}
