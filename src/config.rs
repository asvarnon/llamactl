use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct Config {
    pub models: Vec<ModelEntry>,
}

#[derive(Deserialize)]
pub struct ModelEntry {
    pub name: String, // in double quotes.
    #[serde(default)]
    pub aliases: Vec<String>,
    pub script: String, //in single quotes
}

fn config_path() -> PathBuf {
    // toml file with loaded configs
    PathBuf::from(r"C:\llama.cpp\llm-config.toml")
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let path = config_path();
    if !path.exists() {
        return Err(format!(
            "config file not found at {} — create it to register models",
            path.display()
        )
        .into());
    }
    let contents = std::fs::read_to_string(&path)?;
    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}

pub fn resolve_model<'a>(config: &'a Config, input: &str) -> Option<&'a ModelEntry> {
    let input_lower = input.to_lowercase();
    config.models.iter().find(|m| {
        m.name.to_lowercase() == input_lower
            || m.aliases.iter().any(|a| a.to_lowercase() == input_lower)
    })
}
