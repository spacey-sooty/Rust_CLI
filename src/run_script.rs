use std::vec::Vec;

struct Script {
    Name: String,
    Commands: Vec<String>
}

async fn try_run_script(script: Script) -> Result<()> {
    println!("Running {}", script.Name);

    Ok(())
}