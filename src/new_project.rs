use clap::Args;
use crate::run_script;
use tokio::io::Result;


#[derive(Args)]
pub struct ProjectArgs {}

pub async fn try_new_project() -> Result<()> {
    let create_new_project: run_script::Script = run_script::Script {
        name: String::from("Create New Project"),
        commands: String::from("")
    };

    println!("Create new project starting");

    run_script::try_run_script(create_new_project).await.unwrap();

    Ok(())
}