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
  fs::File::create(confpath)
      .unwrap()
      .write_all(b"[profile.release]\nlto=true\nstrip='debuginfo'\n")
      .unwrap();
  fs::create_dir(format!("{}/src", proj_name)).unwrap();
  fs::File::create(format!("{}/src/lib.rs", proj_name)).unwrap();
  fs::File::create(format!("{}/src/main.rs", proj_name))
      .unwrap()
      .write_all(b"fn main() {}")
      .unwrap();
  fs::File::create(format!("{}/build.rs", proj_name))
      .unwrap()
      .write_all(b"fn main() {}")
      .unwrap();
  fs::File::create(format!("{}/src/tests.rs", proj_name))
      .unwrap()
      .write_all(b"#[cfg(test)]\nmod tests{}\n")
      .unwrap();
  fs::File::create(format!("{}/.gitignore", proj_name))
      .unwrap()
      .write_all(b"/target\nCargo.lock\n*.exe\n*.out\n")
      .unwrap();
  fs::create_dir(format!("{}/.github", proj_name)).unwrap();
  fs::File::create(format!("{}/.github/dependabot.yml", proj_name))
      .unwrap()
      .write_all(b"version: 2\nupdates:\n  - directory: /\n    package-ecosystem: cargo\n    schedule:\n      interval: daily")
      .unwrap();
  fs::create_dir(format!("{}/.github/workflows", proj_name)).unwrap();
  fs::File::create(format!("{}/.github/workflows/ci.yml", proj_name))
      .unwrap()
      .write_all(b"name: Build and Test\n\non:\n  push:\n    branches: ['*']\n  pull_request:\n    branches: ['master']\n\njobs:\n  Ubuntu:\n    runs-on: ubuntu-latest\n    steps:\n    - uses: actions/checkout@v3\n    - name: Build\n      run: cargo build --verbose\n    - name: Run Tests\n      run: cargo test --verbose\n\n  Windows:\n    runs-on: windows-latest\n    steps:\n    - uses: actions/checkout@v3\n    - name: Build\n      run: cargo build --verbose\n    - name: Run Tests\n      run: cargo test --verbose")
      .unwrap();
  Command::new("powershell")
    .arg(format!("cd {}", proj_name))
    .arg(" | ")
    .arg("cargo init")
    .arg(" | ")
    .arg(git)
    .spawn()
    .expect("powershell command failed to start");
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
