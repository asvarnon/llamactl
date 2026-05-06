use crate::config::Config;

pub fn run(config: &Config) {
    println!("{:<20} {}", "NAME", "ALIASES");
    println!("{}", "-".repeat(50));
    for m in &config.models {
        let aliases = if m.aliases.is_empty() {
            "(none)".to_string()
        } else {
            m.aliases.join(", ")
        };
        println!("{:<20} {}", m.name, aliases);
    }
}
