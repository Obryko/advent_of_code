use std::error::Error;

pub mod run;


pub type CommandResult = Result<(), Box<dyn std::error::Error>>;

pub trait Command {
    fn execute(self) -> CommandResult;
}