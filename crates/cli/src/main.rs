pub mod cli;
mod commands;
mod output;

use crate::cli::Cli;
use crate::commands::{Command, CommandResult};
use clap::Parser;
use dotenvy::dotenv;

fn main() -> CommandResult {
    dotenv().ok();
    Cli::parse().command.execute()
}
