mod error;
mod execution;

use crate::commands::CommandOutput;

pub use error::print_error;

pub fn print(output: &CommandOutput) {
    match output {
        CommandOutput::Execution(report) => {
            execution::print_report(report);
        }
    }
}
