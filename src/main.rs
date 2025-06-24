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
    CatFile {
        #[arg(short = 'p')]
        pretty: bool,
        object_hash: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => ferrit::init(),
        Commands::CatFile {
            pretty: true,
            object_hash,
        } => ferrit::cat_file(&object_hash),
        Commands::CatFile { .. } => todo!(),
    }
}
