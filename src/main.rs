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
    HashObject {
        #[arg(short = 'w')]
        write: bool,
        file_name: String,
    },
    LsTree {
        #[arg(long = "name-only")]
        name_only: bool,
        tree_sha: String,
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
        Commands::HashObject {
            write: true,
            file_name,
        } => {
            let hash = ferrit::hash_object(&file_name, true);
            println!("{}", hash);
        }
        Commands::HashObject { .. } => todo!(),
        Commands::LsTree { name_only, tree_sha } => {
            ferrit::ls_tree(&tree_sha, name_only);
        }
    }
}
