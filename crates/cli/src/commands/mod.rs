pub(crate) use crate::commands::result::CommandOutput;
use crate::error::CommandError;

pub mod result;
pub mod run;

pub type CommandResult = Result<CommandOutput, CommandError>;

pub trait Command {
    fn execute(self) -> CommandResult;
}
