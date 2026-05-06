mod commands;
mod config;
mod server;
mod state;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "llamactl", about = "llama.cpp model manager", version)]
struct Cli {
    /// Path to the configuration file
    #[arg(short, long, global = true)]
    config: Option<std::path::PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start a model by name or alias
    Start {
        /// Model name or alias (see `llamactl list`)
        name: String,
    },
    /// Stop the running llama-server
    Stop,
    /// Show running model and server health
    Status,
    /// List configured models and aliases
    List,
    /// Shorthand: `llamactl <name>` starts the named model
    #[command(external_subcommand)]
    Shorthand(Vec<String>),
}

fn main() {
    let cli = Cli::parse();
    if let Err(e) = run(cli) {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    match cli.command {
        Commands::Start { name } => commands::start::run(&name, cli.config.as_deref()),
        Commands::Stop => commands::stop::run(),
        Commands::Status => {
            commands::status::run();
            Ok(())
        }
        Commands::List => {
            let config = config::load_config(cli.config.as_deref())?;
            commands::list::run(&config);
            Ok(())
        }
        Commands::Shorthand(args) => {
            let name = args
                .first()
                .ok_or("usage: llm <name> or llm <subcommand>")?;
            commands::start::run(name, cli.config.as_deref())
        }
    }
}
