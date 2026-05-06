use crate::state::{clear_state, format_elapsed, read_state};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let maybe_state = read_state();

    // Using the 'Sledgehammer' approach (/IM llama-server.exe).
    // Why: The stored PID actually belongs to the batch script runner (cmd.exe).
    // Killing that PID would only stop the shell, potentially leaving the actual 
    // llama-server process running as an orphan. Targeting the image name 
    // ensures the actual workload is terminated.
    let output = std::process::Command::new("taskkill")
        .args(["/F", "/IM", "llama-server.exe", "/T"])
        .output()?;

    clear_state()?;

    if output.status.success() {
        match maybe_state {
            Some(s) => println!(
                "Stopped {} (was running for {})",
                s.model_name,
                format_elapsed(s.started_at)
            ),
            None => println!("Stopped llama-server"),
        }
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains("not found") || stderr.is_empty() {
            println!("No llama-server process found");
        } else {
            println!("taskkill: {}", stderr.trim());
        }
    }

    Ok(())
}
