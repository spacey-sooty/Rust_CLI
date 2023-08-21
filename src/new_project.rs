use clap::{Args, Subcommand};

#[derive(Args)]
pub struct ProjectArgs {
  #[command(subcommand)]
  lang: Langs,
}

#[derive(Subcommand)]
enum Langs {
  Rust,
  Svelte,
}
