mod cli;
mod metrics;
mod runner;

use clap::Parser;

use crate::cli::Cli;
use crate::runner::Runner;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let runner = Runner::new(cli);
    runner.run().await;
}
