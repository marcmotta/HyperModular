// src/main.rs
/*
 * Main executable for HyperModular
 */

use clap::Parser;
use hypermodular::{Result, run};

#[derive(Parser)]
#[command(version, about = "HyperModular - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
