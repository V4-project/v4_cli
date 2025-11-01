use clap::{Parser, Subcommand};

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
    },

    /// Reset VM
    Reset {
        /// Serial port path
        #[arg(short, long)]
        port: String,
    },
}

fn main() {
    let cli = Cli::parse();

    let result: anyhow::Result<()> = match cli.command {
        Commands::Push { file, port, detach, timeout } => {
            println!("Push command: file={}, port={}, detach={}, timeout={}", file, port, detach, timeout);
            Ok(())
        }
        Commands::Ping { port } => {
            println!("Ping command: port={}", port);
            Ok(())
        }
        Commands::Reset { port } => {
            println!("Reset command: port={}", port);
            Ok(())
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
