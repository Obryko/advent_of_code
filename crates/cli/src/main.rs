mod output;
mod commands;
pub mod cli;

use clap::Parser;
use dotenvy::dotenv;
use crate::cli::Cli;
use crate::commands::{Command, CommandResult};

fn main() -> CommandResult {
    dotenv().ok();
    Cli::parse().command.execute()
}
