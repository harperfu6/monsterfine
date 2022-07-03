mod benchmark_result;
mod executor;
mod scheduler;
mod timing_result;

use crate::command::Command;
use crate::options::Options;
use benchmark_result::BenchMarkResult;

use anyhow::Result;

struct BenchMark<'a> {
    number: usize,
    command: Command<'a>,
    opitons: Options,
}

impl<'a> BenchMark<'a> {
    pub fn new(number: usize, command: Command<'a>, opitons: Options) -> Self {
        BenchMark {
            number,
            command,
            opitons,
        }
    }

    pub fn run(&self) -> Result<BenchMarkResult> {
        todo!()
    }
}
