use clap::Parser;

use crate::{cli::Cli, cli::Commands};

pub mod chunk;
pub mod chunk_err;
pub mod chunk_type;
pub mod cli;
pub mod cmd;
pub mod png;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encode(args) => cmd::encode(args),
        Commands::Decode(decode_args) => cmd::decode(decode_args),
        Commands::Remove(remove_args) => cmd::remove(remove_args),
        Commands::Print { file_path } => cmd::print(file_path.clone()),
    }
}
