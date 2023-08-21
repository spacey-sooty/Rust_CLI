use clap::{Parser, Subcommand};
use cli::new_project;

#[derive(Parser)]
#[command(name = "Personal CLI")]
#[command(author = "Isaac Turner")]
#[command(version = "0.1")]
#[command(
  about = "My CLI tool",
  long_about = "A CLI tool I built to do work that I want"
)]

struct Cli {
  #[command(subcommand)]
  command: Commands,
}

#[derive(Subcommand)]
enum Commands {
  Project(new_project::ProjectArgs),
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
  println!("Platform: {}", std::env::consts::OS);

  let cli = Cli::parse();
  match &cli.command {
    Commands::Project(..) => {
      if std::env::consts::OS == "windows" {
        println!("Starting new project, windows");
        todo!("Implement new project");
      } else {
        println!("Starting new project, linux");
        todo!("Implement new project");
      }
    }
  }
}
