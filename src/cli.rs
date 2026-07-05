use clap::{Args, Parser, Subcommand};

use crate::chunk_type::ChunkType;

#[derive(Parser)]
#[command(
    name = "pngnator",
    version = "1.0",
    about = "A steganography tool for png images."
)]
pub struct Cli {
    #[arg(short, long, global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Encode(EncodeArgs),
    Decode(MutateArgs),
    Remove(MutateArgs),
    Print { file_path: String },
}

#[derive(Args)]
pub struct EncodeArgs {
    pub file_path: String,
    pub chunk_type: ChunkType,
    pub message: String,

    #[arg(default_value = "output.png")]
    pub output_file: String,
}

#[derive(Args)]
pub struct MutateArgs {
    pub file_path: String,
    pub chunk_type: ChunkType,
}
