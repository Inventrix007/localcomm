use anyhow::Result;
use clap::Parser;

mod cli;
mod commands;
mod discovery;
mod protocol;
mod transport;
mod ui;

use cli::{Cli, Commands};

#[tokio::main]
async fn main() -> Result<()> {
        // Initialize tracing
        tracing_subscriber::fmt::init();

        let cli =Cli::parse();

        match cli.command {
            Commands::Discover {timeout} => {
                commands::discover::run(timeout).await?;
            }
            Commands::List => {
                commands::list::run().await?;
            }
            Commands::Send { to, message } => {
                commands::send::run(&to, &message).await?;
            }
            Commands::Chat {address} => {
                commands::chat::run(&address).await?;
            }
            Commands::Serve { port, name } => {
                println!("{}", "=== localComm Server ===");
                commands::serve::run(port, name).await?;
            }
            Commands::Transfer { subcommand } => {
                // commands::transfer::run(subcommand).await?;
            }
            Commands::Tui => {
                commands::tui::run().await?;
            }
        } 
        Ok(())
}

