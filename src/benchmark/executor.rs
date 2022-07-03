use anyhow::Result;
use std::process::ExitStatus;

use super::timing_result::TimingResult;
use crate::command::Command;

pub trait Executor {
    /// Run the given command and measure the execuiton time
    fn run_command_and_measure(&self, command: &Command) -> Result<(TimingResult, ExitStatus)>;
}
