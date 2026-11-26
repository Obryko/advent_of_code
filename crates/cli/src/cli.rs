use crate::commands::run::Run;
use crate::commands::{Command, CommandResult};
use clap::{Parser, Subcommand};

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
