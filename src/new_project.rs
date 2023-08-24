use clap::{Args, Subcommand};
use std::process::Command;

#[derive(Args, Debug)]
// #[derive(Debug, Parser)]
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
  println!("starting a new rust project, with name {}", proj_name);
  if cfg!(target_os = "windows") {

  } else {
    todo!("linux implementation");
  }
}

pub fn svelte_project(proj_name: &String) {
    println!("starting a new svelte project, with name {}", proj_name);
    if cfg!(target_os = "windows") {

    } else {
        todo!("linux implementation");
    }
}
