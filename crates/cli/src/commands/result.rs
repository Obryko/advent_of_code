use crate::report::execution::ExecutionReport;

pub enum CommandOutput {
    Execution(ExecutionReport),
}

impl CommandOutput {
    pub fn is_success(&self) -> bool {
        match self {
            Self::Execution(report) => report.is_success(),
        }
    }
}
