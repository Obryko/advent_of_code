use crate::commands::run::Run;
use crate::commands::{Command, CommandResult};
use clap::{CommandFactory, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

impl Cli {
    pub fn parse_validated() -> Self {
        let cli = Self::parse();

        let result = match &cli.command {
            Commands::Run(command) => command.validate(),
        };

        if let Err(error) = result {
            error.format(&mut Self::command()).exit();
        }

        cli
    }
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
