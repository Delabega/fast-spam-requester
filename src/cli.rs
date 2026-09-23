use clap::{ArgAction, Parser};
use std::time::Duration;

#[derive(Parser)]
#[command(name = "fast-spam-requester", alias = "fsq", author, about, version)]
pub struct Cli {
    #[arg(short, long)]
    pub url: String,

    #[arg(short, long, default_value_t = 50)]
    pub concurrency: usize,

    #[arg(short, long, default_value = "10s", value_parser = humantime::parse_duration)]
    pub duration: Duration,

    #[arg(short = 'H', long, action = ArgAction::Append)]
    pub headers: Vec<String>,

    #[arg(short, long)]
    pub body: Option<String>,
}
