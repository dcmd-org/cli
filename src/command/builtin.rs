use std::{io::{Error, Write}, process::{Command, self}, fs::{self, DirEntry, File}, path::Path, env};

use curl::easy::Easy;

use crate::config::Config;

pub fn handle_list(config: &Config) {
  let commands_path = Path::new(config.get_env().get_docker_folder()).join("commands");

  let commands_dir = fs::read_dir(&commands_path)
  .expect("Impossible to access commands directory");

  println!("Available builtin commands: \n");

  super::Command::iter_builtin()
  .for_each(|command| println!("dcmd {}{}", super::get_fixed_length(&command.to_string(), 30), command.get_description()));

  println!("\nAvailable custom commands: \n");

  commands_dir
  .flat_map(|entry| -> Vec<String> {
    let entry = entry.unwrap();
    if entry.file_type().unwrap().is_dir() {
      let app = entry.file_name().into_string().unwrap();
      fs::read_dir(commands_path.join(&app))
      .expect("Impossible to access sub commands directory")
      .flat_map(|entry| Ok::<DirEntry, Error>(entry.unwrap()))
      .map(|entry| {
        let mut command = app.clone();
        command.push(' ');
        command.push_str(entry.file_name().into_string().unwrap().as_str());
        command
      })
      .collect::<Vec<String>>()

    } else {
      Vec::from([entry.file_name().into_string().unwrap()])
    }
  })
  .for_each(|result| {
    let rdesc = regex::Regex::new(r"# ?@description ?: ?(.*)").unwrap();
    let mut file_path = Path::new(config.get_env().get_docker_folder())
    .join("commands");

    result.split(' ')
    .for_each(|f| file_path.push(f));

    let file_content = fs::read_to_string(file_path)
    .expect("Cannot read command file");

    let captures = rdesc
    .captures(&file_content);

    if let Some(captures) = captures {
      if let Some(cap) = captures.get(1) {
        println!("dcmd {}{}", super::get_fixed_length(&result, 30), cap.as_str());
      }
    } else {
      println!("dcmd {}", result);
    }
  });
}

pub fn handle_help() {
  println!("Run dcmd ls to see all available commands");
}

pub fn handle_update(config: &Config) {
  println!("Updating CLI for platform: {}", config.get_env().get_platform());
  let mut url = String::from(env!("GITHUB_RAW_URL"));
  url.push_str("/cli/main/bin/");
  url.push_str(config.get_env().get_platform());
  url.push_str("/dcmd");

  let mut easy = Easy::new();
  easy.url(url.as_str()).unwrap();

  let mut executable = File::create(env::current_exe().unwrap())
  .unwrap_or_else(|_| {
    println!("Can't write to the destination path, please check file permission or run the command as sudoer.");

    process::exit(1);
  });

  let mut transfer = easy.transfer();
  transfer.write_function(|data| {
    executable.write_all(data).expect("Can't write file to the destination path");
    Ok(data.len())
  })
  .expect("Error while writing the file");

  transfer
  .perform()
  .unwrap_or_else(|_| {
    println!("The CLI was not able to fetch data from the remote server, check your internet connection or download the update manually {}/docs/getting-started/cli-installation", env!("WEBSITE_URL"));

    process::exit(1);
  });

  // println!("CLI successfully updated ✅");
}

pub fn handle_version(config: &Config) {
  println!("Docker Commands version: {}", config.get_env().get_version());
  println!("Platform: {}", config.get_env().get_platform());
}

pub fn handle_up(config: &Config) {
  let mut command = Command::new("docker");
  initialize_docker_command(&mut command, config);

  command
  .arg("--build")
  .arg("-d");

  finalize_docker_command(&mut command, config);
  super::exec_command(command);
}

pub fn handle_down(config: &Config) {
  let mut command = Command::new("docker");
  initialize_docker_command(&mut command, config);

  command
  .arg("-v")
  .arg("-t")
  .arg(config.get_env().get_stop_timeout().to_string());

  finalize_docker_command(&mut command, config);
  super::exec_command(command);
}

pub fn handle_stop(config: &Config) {
  let mut command = Command::new("docker");
  initialize_docker_command(&mut command, config);

  command
  .arg("-t")
  .arg(config.get_env().get_stop_timeout().to_string());

  finalize_docker_command(&mut command, config);
  super::exec_command(command);
}

pub fn handle_start(config: &Config) {
  let mut command = Command::new("docker");
  initialize_docker_command(&mut command, config);
  finalize_docker_command(&mut command, config);
  super::exec_command(command);
}

pub fn handle_restart(config: &Config) {
  let mut command = Command::new("docker");
  initialize_docker_command(&mut command, config);
  command
  .arg("-t")
  .arg(config.get_env().get_stop_timeout().to_string());

  finalize_docker_command(&mut command, config);
  super::exec_command(command);
}

fn initialize_docker_command<'a>(command: &'a mut Command, config: &'a Config) -> &'a Command {

  command
  .arg("compose")
  .arg("-f")
  .arg(config.get_env().get_docker_compose_file())
  .arg(config.get_command().to_string())
  .envs(config.get_env().get_docker_env_vars().into_iter());

  command
}

fn finalize_docker_command<'a>(command: &'a mut Command, config: &Config) -> &'a Command {
  config.get_arguments()
  .iter()
  .for_each(|arg| {
    command.arg(arg);
  });

  command
}
