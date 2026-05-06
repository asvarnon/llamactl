use std::sync::LazyLock;
use std::time::Duration;

const HEALTH_URL: &str = "http://127.0.0.1:8080/health";

static AGENT: LazyLock<ureq::Agent> = LazyLock::new(|| {
    ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(2)))
        .build()
        .new_agent()
});

pub enum HealthResult {
    Ok(String),
    Unreachable,
    Error(String),
}

pub fn get_health() -> HealthResult {
    match AGENT.get(HEALTH_URL).call() {
        Ok(mut resp) => match resp.body_mut().read_to_string() {
            Ok(body) => HealthResult::Ok(body),
            Err(e) => HealthResult::Error(e.to_string()),
        },
        Err(ureq::Error::Io(_)) => HealthResult::Unreachable,
        Err(e) => HealthResult::Error(e.to_string()),
    }
}

pub fn poll_until_healthy(timeout_secs: u64) -> bool {
    let iterations = timeout_secs * 2;
    print!("Waiting for server");
    for _ in 0..iterations {
        std::thread::sleep(Duration::from_millis(500));
        print!(".");
        use std::io::Write;
        let _ = std::io::stdout().flush();
        if matches!(get_health(), HealthResult::Ok(_)) {
            println!();
            return true;
        }
    }
    println!();
    false
}
