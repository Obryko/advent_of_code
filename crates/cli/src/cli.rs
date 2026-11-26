use std::error::Error;
use clap::{Parser, Subcommand};
use crate::commands::{Command, CommandResult};
use crate::commands::run::Run;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Runs the specified Advent of Code solution
    Run(Run),
}

impl Command for Commands {
    fn execute(self) -> CommandResult {
        match self {
            Commands::Run(command) => Command::execute(command),
        }
    }
}