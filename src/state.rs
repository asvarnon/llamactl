use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize)]
pub struct RunState {
    pub model_name: String,
    pub script_path: String,
    pub pid: u32,
    pub started_at: u64,
}

pub fn state_path() -> PathBuf {
    std::env::temp_dir().join("llm-state.json")
}

pub fn read_state() -> Option<RunState> {
    let contents = std::fs::read_to_string(state_path()).ok()?;
    serde_json::from_str(&contents).ok()
}

pub fn write_state(state: &RunState) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(state)?;
    std::fs::write(state_path(), json)?;
    Ok(())
}

pub fn clear_state() -> Result<(), Box<dyn std::error::Error>> {
    let path = state_path();
    if path.exists() {
        std::fs::remove_file(path)?;
    }
    Ok(())
}

pub fn unix_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

pub fn format_elapsed(started_at: u64) -> String {
    let elapsed = unix_now().saturating_sub(started_at);
    let h = elapsed / 3600;
    let m = (elapsed % 3600) / 60;
    let s = elapsed % 60;
    if h > 0 {
        format!("{}h {}m", h, m)
    } else if m > 0 {
        format!("{}m {}s", m, s)
    } else {
        format!("{}s", s)
    }
}

// checks for signal (PID) existence
pub fn pid_alive(pid: u32) -> bool {
    let output = std::process::Command::new("tasklist")
        .args(["/FI", &format!("PID eq {}", pid), "/NH", "/FO", "CSV"])
        .output();
    match output {
        Ok(o) => {
            let s = String::from_utf8_lossy(&o.stdout);
            s.contains(&pid.to_string())
        }
        _ => false,
    }
}
