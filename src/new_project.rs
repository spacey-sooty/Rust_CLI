use clap::{Args, Subcommand};
use std::fs;
use std::fs::File;

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
  println!("starting a new rust project, with name {}", proj_name);
}

pub fn svelte_project(proj_name: &String) {
  println!("starting a new svelte project, with name {}", proj_name);
  fs::create_dir(&proj_name).unwrap();
}
