use std::{process::{Command, self}, fs, env};

use fs_extra::dir::CopyOptions;

use crate::config::Config;


pub fn handle_template(config: &Config) {
  let action = config.get_arguments().get(0).unwrap_or_else(|| {
    println!("You must specify an action, ex: template get");
    process::exit(1);
  });

  match action.as_str() {
    "get" => get_template(config),
    _ => {
      println!("You must specify a valid argument, for example get [template_name]");
      process::exit(1);
    }
  }
}

fn get_template(config: &Config) {

  let template_name = config.get_arguments().get(1).unwrap_or_else(|| {
    println!("You must specify a template to use, see more information here: {}/docs/category/templates", env!("WEBSITE_URL"));
    process::exit(1);
  });

  // Create .dcmd dir if it does not exist yet
  let mut templates_repository = String::from(env!("GITHUB_URL"));
  templates_repository.push_str("/templates.git");

  let templates_path = home::home_dir()
  .unwrap()
  .join(".dcmd/templates");

  println!("Fetching latest templates from remote...");

  if !templates_path.exists() {

    fs::create_dir_all(&templates_path)
    .expect("Can't create ~/.dcmd directory, check file permissions");

    Command::new("git")
    .arg("clone")
    .arg(templates_repository.as_str())
    .arg(templates_path.to_str().unwrap())
    .arg("--quiet")
    .status()
    .expect("Cannot run git clone command");

  } else {

    Command::new("git")
    .arg("pull")
    .arg("origin")
    .arg("main")
    .arg("--quiet")
    .current_dir(templates_path.to_str().unwrap())
    .status()
    .expect("Cannot run git pull command");

  }

  println!("Templates have been updated");

  // Check id template exists
  let template_path = templates_path
  .join(template_name);

  if !template_path.exists() {
    println!("This template does not exist, please check the web for more informations: {}/docs/category/templates", env!("WEBSITE_URL"));
    process::exit(1);
  }

  println!("copying the template in {}/.docker", env::current_dir().unwrap().to_str().unwrap());

  let copy_from = template_path.join(".docker");
  let copy_dest = env::current_dir().unwrap().join(".docker");

  if copy_dest.is_dir() {
    if casual::confirm(".docker directory will be replaced, continue?") {
      fs::remove_dir_all(&copy_dest)
      .expect("Cannot delete the directory, please check file permissions");
    } else {
      process::exit(1);
    }
  }

  let mut options = CopyOptions::new();
  options.overwrite = true;
  options.copy_inside = true;

  fs_extra::dir::copy(copy_from, copy_dest, &options)
  .expect("Cannot copy the template in current directory, please check file permissions");

  println!("✅ Template successfully created, do not forget to configure it before running the up command.")
}
