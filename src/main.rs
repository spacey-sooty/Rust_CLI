use clap::{Parser, Subcommand};
use cli::new_project::ProjectArgs;
use cli::new_project;

#[derive(Parser)]
#[command(name = "Personal CLI")]
#[command(author = "Isaac Turner")]
#[command(version = "0.1")]
#[command(
  about = "My CLI tool",
  long_about = "A CLI tool I built to perform tasks I commonly require",
)]

struct Cli {
  #[command(subcommand)]
  command: Commands,
}

#[derive(Subcommand)]
enum Commands {
  New(ProjectArgs),
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
  println!("Platform: {}", std::env::consts::OS);

  let cli = Cli::parse();
  match &cli.command {
    Commands::New(project_args) => {
        match &project_args.lang {
            new_project::Langs::Rust => {
                new_project::rust_project(&project_args.name);
            }
            new_project::Langs::Svelte => {
                new_project::svelte_project(&project_args.name);
            }
        }
    }
  }
}
