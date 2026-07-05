use clap::{Args, Parser, Subcommand};

use crate::chunk_type::ChunkType;

pub mod chunk;
pub mod chunk_err;
pub mod chunk_type;
pub mod png;

#[derive(Parser)]
#[command(
    name = "pngnator",
    version = "1.0",
    about = "A steganography tool for png images."
)]
struct Cli {
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Encode(EncodeArgs),
    Decode(MutateArgs),
    Remove(MutateArgs),
    Print { file_path: String },
}

#[derive(Args)]
struct EncodeArgs {
    file_path: String,
    chunk_type: ChunkType,
    message: String,

    #[arg(default_value = "output.png")]
    output_file: String,
}

#[derive(Args)]
struct MutateArgs {
    file_path: String,
    chunk_type: ChunkType,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Encode(EncodeArgs {
            file_path,
            chunk_type,
            message,
            output_file,
        }) => {}
        Commands::Decode(MutateArgs {
            file_path,
            chunk_type,
        }) => {}
        Commands::Remove(MutateArgs {
            file_path,
            chunk_type,
        }) => {}
        Commands::Print { file_path } => {}
    }
}
