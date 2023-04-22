use std::{process::{Command, self}, path::Path};

use crate::config::Config;


pub fn handle_custom(config: &Config) {
  let mut command = Command::new("bash");
  command.envs(config.get_env().get_docker_env_vars().into_iter());

  let custom_command_path = Path::new(config.get_env().get_docker_folder())
  .join("commands")
  .join(config.get_command().to_string());

  let is_nested: bool = match config.get_arguments().get(0) {
    Some(arg) => {
      let arg_as_file = Path::is_absolute(Path::new(arg));
      let exists = Path::new(&custom_command_path)
      .join(arg)
      .is_file();
      !arg_as_file && exists
    },
    _ => false
  };

  if is_nested {
    let custom_command_path = custom_command_path
    .join(config.get_arguments().get(0).unwrap());
    let args = &mut config.get_arguments().clone()[1..];
    command
    .arg(&custom_command_path)
    .args(args);

    if !Path::new(&custom_command_path).is_file() {
      println!("Custom command {} does not exists", custom_command_path.to_str().unwrap());
      process::exit(1);
    }

  } else {
    command
    .arg(&custom_command_path)
    .args(config.get_arguments());

    if !Path::new(&custom_command_path).is_file() {
      println!("Custom command {} does not exists", custom_command_path.to_str().unwrap());
      process::exit(1);
    }
  }


  super::exec_command(command);

}
