// src/main.rs
/*
 * Main executable for ArtificialIntuitionFramework
 */

use clap::Parser;
use artificialintuitionframework::{Result, run};

#[derive(Parser)]
#[command(version, about = "ArtificialIntuitionFramework - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
