use std::vec::Vec;
use tokio::io::Result;

pub struct Script {
    pub name: String,
    pub commands: Vec<String>
}

pub async fn try_run_script(script: Script) -> Result<()> {
    println!("Running {}", script.name);

    Ok(())
}