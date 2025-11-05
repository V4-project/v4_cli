use clap::{Parser, Subcommand};
use std::time::Duration;
use v4_cli::commands;

#[derive(Parser)]
#[command(name = "v4")]
#[command(version, about = "CLI tool for V4 VM bytecode deployment", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Deploy bytecode to device
    Push {
        /// Bytecode file path
        file: String,

        /// Serial port path (e.g., /dev/ttyACM0)
        #[arg(short, long)]
        port: String,

        /// Don't wait for response
        #[arg(long)]
        detach: bool,

        /// Timeout in seconds
        #[arg(long, default_value = "5")]
        timeout: u64,
    },

    /// Check connection to device
    Ping {
        /// Serial port path
        #[arg(short, long)]
        port: String,

        /// Timeout in seconds
        #[arg(long, default_value = "5")]
        timeout: u64,
    },

    /// Reset VM
    Reset {
        /// Serial port path
        #[arg(short, long)]
        port: String,

        /// Timeout in seconds
        #[arg(long, default_value = "5")]
        timeout: u64,
    },

    /// Compile Forth source to bytecode
    Compile {
        /// Input Forth source file path
        input: String,

        /// Output bytecode file path (default: input with .v4b extension)
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Start interactive REPL session
    Repl {
        /// Serial port path (e.g., /dev/ttyACM0)
        #[arg(short, long)]
        port: String,

        /// Skip VM reset on startup (preserves existing words)
        #[arg(long)]
        no_reset: bool,
    },

    /// Execute Forth source file on device
    Exec {
        /// Forth source file path
        file: String,

        /// Serial port path (e.g., /dev/ttyACM0)
        #[arg(short, long)]
        port: String,

        /// Timeout in seconds
        #[arg(long, default_value = "5")]
        timeout: u64,

        /// Enter REPL after execution
        #[arg(long)]
        repl: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Push {
            file,
            port,
            detach,
            timeout,
        } => commands::push(&file, &port, detach, Duration::from_secs(timeout)),

        Commands::Ping { port, timeout } => commands::ping(&port, Duration::from_secs(timeout)),

        Commands::Reset { port, timeout } => commands::reset(&port, Duration::from_secs(timeout)),

        Commands::Compile { input, output } => {
            commands::compile(&input, output.as_deref())
        }

        Commands::Repl { port, no_reset } => commands::run_repl(&port, no_reset),

        Commands::Exec {
            file,
            port,
            timeout,
            repl,
        } => commands::exec(&file, &port, Duration::from_secs(timeout), repl),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
