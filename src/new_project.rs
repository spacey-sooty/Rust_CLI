use clap::{Args, Subcommand};
use std::fs;
use std::process::Command;

#[derive(Args, Debug)]
pub struct ProjectArgs {
  #[command(subcommand)]
  pub lang: Langs,
  #[arg(long, short)]
  pub name: String,
}

#[derive(Subcommand, Debug)]
pub enum Langs {
  Rust,
  Svelte,
}

pub fn rust_project(proj_name: &String) {
  let git = r#"git init | git add . | git commit -m "Initial Commit""#;
  println!("starting a new rust project, with name {}", proj_name);
  fs::create_dir(proj_name).unwrap();
  fs::create_dir(format!("{}/.cargo", proj_name)).unwrap();
  Command::new("powershell")
    .arg(format!("cd {}", proj_name))
    .arg(" | ")
    .arg("cargo init")
    .arg(" | ")
    .arg(git)
    .spawn()
    .expect("cargo command failed to start");
}

pub fn svelte_project(proj_name: &String) {
  println!("starting a new svelte project, with name {}", proj_name);
}
