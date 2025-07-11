// src/main.rs
/*
 * Main executable for CryptoShieldFramework
 */

use clap::Parser;
use cryptoshieldframework::{Result, run};

#[derive(Parser)]
#[command(version, about = "CryptoShieldFramework - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
