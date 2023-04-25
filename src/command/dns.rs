use std::process::Command;

use crate::config::Config;


pub fn handle_dns(config: &Config) {
  let action = config
  .get_arguments()
  .get(0)
  .expect("Please provide a command for dns [start|stop]");

  match action.as_str() {
    "start" => handle_dns_start(),
    "stop" => handle_dns_stop(),
    _ => println!("Please provide a valid parameter such as start or stop")
  }
}

fn handle_dns_start() {
  let mut command = Command::new("docker");
  command
  .arg("run")
  .arg("--name")
  .arg("dcmd_dns")
  .arg("-p")
  .arg("53:53/udp")
  .arg("--rm")
  .arg("-d")
  .arg(env!("DNS_IMAGE"));

  println!("Launching the DNS container...");

  super::exec_command(command);
}

fn handle_dns_stop() {
  let mut command = Command::new("docker");
  command
  .arg("stop")
  .arg("dcmd_dns");

  println!("Stopping the DNS container...");

  super::exec_command(command);
}