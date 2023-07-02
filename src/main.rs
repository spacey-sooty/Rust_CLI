use clap::{Args, Parser, Subcommand};
use cli::new_project;
use tokio::io::Result;

#[derive(Parser)]
#[command(name = "CLI")]
#[command(author = "Isaac Turner")]
#[command(version = "0.1")]
#[command(
    about = "",
    long_about = ""
)]

struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Start(StartArgs),
    New_Project(ProjectArgs),
}

#[derive(Args)]
struct StartArgs {}

async fn try_start() -> Result<()> {
    println!("Hello World!");

    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("Platform: {}", std::env::consts::OS);

    let cli = Cli::parse();
    match &cli.command {
        Commands::Start(..) => {
            try_start().await.unwrap();
        }
        Commands::New_Project(..) => {
            new_project::try_new_project().await.unwrap();
        }
    }
}