use clap::Args;
use crate::run_script;

#[derive(Args)]
struct ProjectArgs;

async fn try_new_project() -> Result<()> {
    let create_new_project: run_script::Script = run_script::Script {
        Name: "Create New Project",
        Commands: ""
    };

    println!("Create new project starting");

    run_script::try_run_script(create_new_project);

    ok(())
}