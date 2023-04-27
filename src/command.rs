use std::{slice::Iter, fmt::{Display, self}};
use crate::config::Config;

pub mod builtin;
pub mod custom;
pub mod dns;
pub mod template;
pub mod toolbox;
pub mod proxy;

#[derive(Debug)]
pub enum Command<'a> {
  Start(&'a str),
  Stop(&'a str),
  Restart(&'a str),
  Up(&'a str),
  Down(&'a str),
  List(&'a str),
  Custom(&'a str),
  Update(&'a str),
  Help(&'a str),
  Version(&'a str),
  Proxy(&'a str),
  Template(&'a str),
  Toolbox(&'a str),
  Dns(&'a str),
}

pub enum CommandType {
  Project,
  System,
}

impl<'a> Clone for Command<'a> {
  fn clone(&self) -> Self {
    match *self {
      Command::Start(value) => Command::Start(value),
      Command::Stop(value) => Command::Stop(value),
      Command::Restart(value) => Command::Restart(value),
      Command::Up(value) => Command::Up(value),
      Command::Down(value) => Command::Down(value),
      Command::List(value) => Command::List(value),
      Command::Custom(value) => Command::Custom(value),
      Command::Update(value) => Command::Update(value),
      Command::Help(value) => Command::Help(value),
      Command::Version(value) => Command::Version(value),
      Command::Proxy(value) => Command::Proxy(value),
      Command::Template(value) => Command::Template(value),
      Command::Toolbox(value) => Command::Toolbox(value),
      Command::Dns(value) => Command::Custom(value),
    }
  }
}

impl<'a> Display for Command<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Command::Start(value) => write!(f, "{}", value),
      Command::Stop(value) => write!(f, "{}", value),
      Command::Restart(value) => write!(f, "{}", value),
      Command::Up(value) => write!(f, "{}", value),
      Command::Down(value) => write!(f, "{}", value),
      Command::List(value) => write!(f, "{}", value),
      Command::Custom(value) => write!(f, "{}", value),
      Command::Update(value) => write!(f, "{}", value),
      Command::Help(value) => write!(f, "{}", value),
      Command::Version(value) => write!(f, "{}", value),
      Command::Proxy(value) => write!(f, "{}", value),
      Command::Template(value) => write!(f, "{}", value),
      Command::Toolbox(value) => write!(f, "{}", value),
      Command::Dns(value) => write!(f, "{}", value),
    }
  }
}

impl Command<'static> {
  pub fn iter_builtin() -> Iter<'static, Command<'static>> {
    static COMMANDS: [Command; 13] = [
      Command::Start("start"),
      Command::Stop("strop"),
      Command::Restart("restart"),
      Command::Up("up"),
      Command::Down("down"),
      Command::Help("help"),
      Command::Version("version"),
      Command::List("list"),
      Command::Update("update"),
      Command::Proxy("proxy"),
      Command::Template("template"),
      Command::Toolbox("toolbox"),
      Command::Dns("dns"),
    ];
    COMMANDS.iter()
  }
}

impl<'a> Command<'a> {
  pub fn get_type(&self) -> CommandType {
    match *self {
      Command::Start(_) => CommandType::Project,
      Command::Stop(_) => CommandType::Project,
      Command::Restart(_) => CommandType::Project,
      Command::Up(_) => CommandType::Project,
      Command::Down(_) => CommandType::Project,
      Command::List(_) => CommandType::Project,
      Command::Custom(_) => CommandType::Project,
      Command::Update(_) => CommandType::System,
      Command::Help(_) => CommandType::System,
      Command::Version(_) => CommandType::System,
      Command::Proxy(_) => CommandType::System,
      Command::Template(_) => CommandType::System,
      Command::Toolbox(_) => CommandType::System,
      Command::Dns(_) => CommandType::System,
    }
  }

  pub fn get_description(&self) -> &str {
    match self {
      Command::Start(_) => "Start a project or a given service",
      Command::Stop(_) => "Stop a project or a given service",
      Command::Restart(_) => "Restart a project or a given service",
      Command::Up(_) => "Initialise a project or a service, should be ran again if you make a changes in your compose file",
      Command::Down(_) => "Remove project containers and related data",
      Command::Help(_) => "Show help",
      Command::Version(_) => "Show the current version",
      Command::List(_) => "List available commands",
      Command::Update(_) => "Update the CLI executable",
      Command::Proxy(_) => "Proxy operations [start|stop]",
      Command::Template(_) => "Template operations [get] [template_name]",
      Command::Toolbox(_) => "Open a toolbox container with the current dir as mount",
      Command::Dns(_) => "DNS operations [start|stop]",
      _ => ""
    }
  }
}

pub fn handle(config: &Config) {
  match config.get_command() {
    Command::Start(_) => builtin::handle_start(config),
    Command::Stop(_) => builtin::handle_stop(config),
    Command::Restart(_) => builtin::handle_restart(config),
    Command::Up(_) => builtin::handle_up(config),
    Command::Down(_) => builtin::handle_down(config),
    Command::List(_) => builtin::handle_list(config),
    Command::Custom(_) => custom::handle_custom(config),
    Command::Update(_) => builtin::handle_update(config),
    Command::Help(_) => builtin::handle_help(),
    Command::Version(_) => builtin::handle_version(config),
    Command::Proxy(_) => proxy::handle_proxy(config),
    Command::Template(_) => template::handle_template(config),
    Command::Toolbox(_) => toolbox::handle_toolbox(config),
    Command::Dns(_) => dns::handle_dns(config),
  }
}

pub fn get_arguments(args: &Vec<String>) -> Vec<String> {
  if args.len() >= 2 {
    return Vec::from(&args[2..])
  }
  Vec::new()
}

pub fn get_command<'a>(args: &'a [String], default: &'a String) -> Command<'a> {
  match args.get(1).unwrap_or(default).as_str() {
    "start" => Command::Start("start"),
    "stop" => Command::Stop("stop"),
    "restart" => Command::Restart("restart"),
    "up" => Command::Up("up"),
    "down" => Command::Down("down"),
    "update" => Command::Update("update"),
    "version" => Command::Version("version"),
    "-v" => Command::Version("version"),
    "--version" => Command::Version("version"),
    "help" => Command::Help("help"),
    "--help" => Command::Help("help"),
    "list" => Command::List("list"),
    "ls" => Command::List("list"),
    "--ls" => Command::List("list"),
    "dns" => Command::Dns("dns"),
    "proxy" => Command::Proxy("proxy"),
    "toolbox" => Command::Toolbox("toolbox"),
    "template" => Command::Template("template"),
    custom => Command::Custom(custom)
  }
}

fn get_fixed_length(value: &String, length: u16) -> String {
  let mut formatted = value.to_string();
  while formatted.len() < length.into() {
    formatted.push(' ');
  }

  formatted
}

fn exec_command(mut command: std::process::Command) {
  command
  .status()
  .unwrap_or_else(|_| {
    println!("Something wrong happend while trying to run command {:?}", command.get_args());
    std::process::exit(1);
  });
}