use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "ferrit")]
#[command(about = "A mini Git-like VCS in Rust", long_about = None)]

struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Init,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => ferrit::init(),
    }
}
