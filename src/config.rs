use serde::Deserialize;
use std::env;
use std::path::{Path, PathBuf};

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

pub fn load_config(custom_path: Option<&Path>) -> Result<Config, Box<dyn std::error::Error>> {
    // Configuration Hierarchy:
    // 1. Explicit path provided via CLI argument
    // 2. Environment variable LLM_CONFIG_PATH
    // 3. Local file 'llm-config.toml' in the current working directory

    let path = if let Some(p) = custom_path {
        p.to_path_buf()
    } else if let Ok(env_path) = env::var("LLM_CONFIG_PATH") {
        PathBuf::from(env_path)
    } else {
        PathBuf::from("llm-config.toml")
    };

    if !path.exists() {
        return Err(format!(
            "config file not found at {} — please specify it using --config or LLM_CONFIG_PATH",
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
