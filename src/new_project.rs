use clap::{Args, Subcommand};
use std::fs;
use std::io::Write;
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

#[cfg(windows)]
pub fn rust_project(proj_name: &String) {
  let git = r#"git init | git add . | git commit -m "Initial Commit""#;
  let confpath = format!("{}/.cargo/config.toml", proj_name);
  println!("starting a new rust project, with name {}", proj_name);
  fs::create_dir(proj_name).unwrap();
  fs::create_dir(format!("{}/.cargo", proj_name)).unwrap();
  fs::File::create(confpath).unwrap().write_all(b"[profile.release]\nlto=true\nstrip='debuginfo'").unwrap();
  Command::new("powershell")
    .arg(format!("cd {}", proj_name))
    .arg(" | ")
    .arg("cargo init")
    //.arg(" | ")
    //.arg("New-Item -Name src/lib.rs -ItemType File")
    .arg(" | ")
    .arg(git)
    .spawn()
    .expect("cargo command failed to start");
}

#[cfg(windows)]
pub fn svelte_project(proj_name: &String) {
  println!("starting a new svelte project, with name {}", proj_name);
}

#[cfg(not(windows))]
pub fn rust_project(proj_name: &String) {
  println!("starting a new rust project, with name {}", proj_name);
}

#[cfg(not(windows))]
pub fn svelte_project(proj_name: &String) {
  println!("starting a new svelte project, with name {}", proj_name);
}
