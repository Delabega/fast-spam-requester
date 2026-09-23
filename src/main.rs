mod cli;
mod metrics;

use clap::Parser;

use crate::cli::Cli;

fn main() {
    let cli = Cli::parse();
    println!("{}", cli.url)
}
