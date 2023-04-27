use std::{process::{Command, self}, path::Path, fs};

use crate::config::Config;


pub fn handle_proxy(config: &Config) {
  let action = config
  .get_arguments()
  .get(0)
  .expect("Please provide a command for proxy [start|stop]");

  match action.as_str() {
    "start" => handle_proxy_start(config),
    "stop" => handle_proxy_stop(),
    _ => println!("Please provide a valid parameter such as start or stop")
  }
}

fn handle_proxy_start(config: &Config) {

  let conf_file = config
  .get_arguments()
  .get(1)
  .expect("The path to the configuration file must be specified");

  let absolute_conf_file = fs::canonicalize(Path::new(conf_file))
  .expect("Can't create an absolute path from the specified parameter");

  if !absolute_conf_file.exists() {
    println!("The specified file does not exist or is not accessible");
    process::exit(1);
  }

  let mut volume = String::from(absolute_conf_file.to_str().unwrap());
  volume.push_str(":/usr/local/apache2/conf/extra/httpd-vhosts.conf");

  let mut command = Command::new("docker");

  command
  .arg("run")
  .arg("--name")
  .arg("dcmd_proxy")
  .arg("-v")
  .arg(volume.as_str())
  .arg("-p")
  .arg("80:80")
  .arg("-p")
  .arg("443:443")
  .arg("--rm")
  .arg("-d")
  .arg(env!("PROXY_IMAGE"));

  println!("Launching the proxy container...");

  super::exec_command(command);
}

fn handle_proxy_stop() {
  let mut command = Command::new("docker");
  command
  .arg("stop")
  .arg("dcmd_proxy");

  println!("Stopping the proxy container...");

  super::exec_command(command);
}