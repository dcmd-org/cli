use std::{process::Command, env};

use crate::config::Config;

pub fn handle_toolbox(config: &Config) {
  let action = config
  .get_arguments()
  .get(0)
  .expect("Please provide a command for proxy [start|update]");

  match action.as_str() {
    "start" => handle_toolbox_start(config),
    "update" => handle_toolbox_update(),
    _ => println!("Please provide a valid parameter such as start or update")
  }
}

fn handle_toolbox_start(_config: &Config) {
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

fn handle_toolbox_update() {
  let mut command = Command::new("docker");
  command
  .arg("pull")
  .arg(env!("TOOLBOX_IMAGE"));

  println!("Pulling the toolbox container...");

  super::exec_command(command);
}
