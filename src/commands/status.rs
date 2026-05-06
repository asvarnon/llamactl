use crate::server::{get_health, HealthResult};
use crate::state::{format_elapsed, pid_alive, read_state};

pub fn run() {
    match read_state() {
        None => {
            println!("No model running (no state file)");
            print_health();
        }
        Some(state) => {
            let alive = pid_alive(state.pid);
            println!("Model:   {}", state.model_name);
            println!("PID:     {} ({})", state.pid, if alive { "alive" } else { "DEAD" });
            println!("Uptime:  {}", format_elapsed(state.started_at));
            print_health();
        }
    }
}

fn print_health() {
    match get_health() {
        HealthResult::Ok(body) => println!("/health: {}", body.trim()),
        HealthResult::Unreachable => println!("/health: unreachable"),
        HealthResult::Error(e) => println!("/health: error — {}", e),
    }
}
