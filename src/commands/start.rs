use crate::config::{load_config, resolve_model};
use crate::server::poll_until_healthy;
use crate::state::{unix_now, write_state, RunState};
use std::os::windows::process::CommandExt;

const DETACHED_PROCESS: u32 = 0x00000008;
const CREATE_NEW_PROCESS_GROUP: u32 = 0x00000200;
const CREATE_NO_WINDOW: u32 = 0x08000000;

pub fn run(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;

    let model = resolve_model(&config, name).ok_or_else(|| {
        format!("unknown model \"{}\" — run 'llm list' to see available models", name)
    })?;

    println!("Starting {}...", model.name);

    let child = std::process::Command::new("cmd")
        .args(["/C", &model.script])
        .creation_flags(DETACHED_PROCESS | CREATE_NEW_PROCESS_GROUP | CREATE_NO_WINDOW)
        .spawn()?;

    let pid = child.id();

    write_state(&RunState {
        model_name: model.name.clone(),
        script_path: model.script.clone(),
        pid,
        started_at: unix_now(),
    })?;

    if poll_until_healthy(10) {
        println!("Server ready at http://127.0.0.1:8080");
    } else {
        println!("Timed out waiting for /health — server may still be loading");
    }

    Ok(())
}
