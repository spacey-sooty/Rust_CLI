use clap::{Args, Parser, Subcommand};

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
  Project(ProjectArgs),
}

#[derive(Args)]
struct ProjectArgs {}

#[tokio::main(flavor = "current_thread")]
async fn main() {
  println!("Platform: {}", std::env::consts::OS);

  let cli = Cli::parse();
  match &cli.command {
    Commands::Project(..) => {
      println!("Starting new project");
      todo!("Implement new project");
    }
  }
}
