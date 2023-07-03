use tokio::io::Result;
use std::process::Command;

pub struct Script {
    pub name: String,
    pub commands: String
}

pub async fn try_run_script(script: Script) -> Result<()> {
    println!("Running {}", script.name);

    if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(["/C", "echo Hello World"])
                // .arg("echo Hello World")
                // .args(["cd ..", "cd ..", "cd src" ,"./bin/new_project.bat"])
                .output()
                .expect("failed to execute process")
    }
    else {
        Command::new("sh")
                .arg("echo Hello World")
                .arg("./bin/new_project.sh")
                .output()
                .expect("Failed to execute program")
    };

    Ok(())
}