use std::{process::{Command}, env};

use crate::config::Config;

pub fn handle_toolbox(_config: &Config) {
  let mut command = Command::new("docker");
  let current_dir = env::current_dir().unwrap().to_str().unwrap().to_string();

  command
  .arg("run")
  .arg("-it")
  .arg("--rm")
  .arg("--name")
  .arg("dcmd_toolbox")
  .arg("-v")
  .arg(format!("{}:/app", current_dir))
  .arg(env!("TOOLBOX_IMAGE"));

  super::exec_command(command);
}