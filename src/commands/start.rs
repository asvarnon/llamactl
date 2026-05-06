use crate::config::{load_config, resolve_model};
use crate::server::poll_until_healthy;
use crate::state::{unix_now, write_state, RunState};
use std::fs::OpenOptions;
use std::os::windows::process::CommandExt;

const CREATE_NEW_PROCESS_GROUP: u32 = 0x00000200;
const CREATE_NO_WINDOW: u32 = 0x08000000;

pub fn log_path() -> std::path::PathBuf {
    std::env::temp_dir().join("llama-server.log")
}

pub fn run(
    name: &str,
    config_path: Option<&std::path::Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config(config_path.as_deref())?; //get list of models, alias', and scripts

    let model = resolve_model(&config, name).ok_or_else(|| {
        format!(
            "unknown model \"{}\" — run 'llamactl list' to see available models",
            name
        )
    })?;

    println!("Starting {}...", model.name);

    //creates log file at path.
    let log = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(log_path())?;
    let log_err = log.try_clone()?;

    //the launcher: opens new cmd terminal and runs the command
    let child = std::process::Command::new("cmd")
        .args(["/C", &model.script])
        //.stout and .stderr are the what happened since PID only checks if process is still running, not what happened.
        .stdout(log)
        .stderr(log_err)
        .creation_flags(CREATE_NEW_PROCESS_GROUP | CREATE_NO_WINDOW)
        .spawn()?;

    // Note: This PID refers to the 'cmd.exe' batch runner, not the actual 'llama-server.exe'
    // process inside the script. To target the server directly by PID, we would need
    // to trace the process tree to find the child of this shell.
    let pid = child.id();

    write_state(&RunState {
        model_name: model.name.clone(),
        script_path: model.script.clone(),
        pid,
        started_at: unix_now(),
    })?;

    println!("Log: {}", log_path().display());

    // waiting room, cli stays until confirmed its running
    if poll_until_healthy(30) {
        println!("Server ready at http://127.0.0.1:8080");
    } else {
        println!("Timed out — check log: {}", log_path().display());
    }

    Ok(())
}
