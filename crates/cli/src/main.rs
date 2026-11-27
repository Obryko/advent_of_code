mod cli;
mod commands;
mod error;
mod output;
mod report;

use crate::cli::Cli;
use crate::commands::Command;
use dotenvy::dotenv;
use std::process::ExitCode;

fn main() -> ExitCode {
    dotenv().ok();
    match Cli::parse_validated().command.execute() {
        Ok(result) => {
            output::print(&result);

            if result.is_success() {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            }
        }

        Err(error) => {
            output::print_error(&error);

            ExitCode::FAILURE
        }
    }
}
