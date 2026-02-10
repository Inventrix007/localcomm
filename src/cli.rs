use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "localcomm")]
#[command(author, version, about, long_about = None)]
#[command(arg_required_else_help = true)]
pub struct Cli{
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Discover devices on the local network
    Discover {
        // Discover timeout in seconds
        #[arg(short, long, default_value_t = 5)]
        timeout: u64,
    },
    // List all discoverd devices
    List,

    //Send a message to a device
    Send {
        // Targer device address (host:port)
        #[arg(short, long)]
        to: String,

        // message content
        #[arg(short, long)]
        message: String,
    },

    // Start interactive chat with a device
    Chat {
        // target device address (host:port)
        address: String,
    },

    // Run as server to receive messages
    Serve {
        // Port to liston on
        #[arg(short, long, default_value_t = 8080)]
        port: u16,

        // Device name (default: hostname)
        #[arg(short, long)]
        name: Option<String>,
    },

    //Transfer files
    Transfer {
        #[command(subcommand)]
        subcommand: TransferCommands,
    },

    // launch interactive TUI mode
    Tui,
}

#[derive(Subcommand)]
pub enum TransferCommands {
    // Send a file
    Send {
        // File path to send
        #[arg(short, long)]
        file: String,

        //Target device address (host:port)
        #[arg(short, long)]
        to: String,
    },

    //Receive a file
    Receive {
        //Port to listen on
        #[arg(short, long, default_value_t = 9090)]
        port: u16,

        // Output directory
        #[arg(short, long, default_value = ".")]
        output: String,
    },
}