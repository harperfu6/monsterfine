use super::benchmark_result::BenchMarkResult;
use super::BenchMark;
use crate::command::Commands;
use crate::options::Options;

use anyhow::Result;

pub struct Scheduler<'a> {
    commands: Commands<'a>,
    options: Options,
    results: Vec<BenchMarkResult>,
}

impl<'a> Scheduler<'a> {
    pub fn new(commands: Commands<'a>, options: Options) -> Self {
        Scheduler {
            commands,
            options,
            results: vec![],
        }
    }

    pub fn run_benchmarks(&self) -> Result<()> {
        for (number, cmd) in self.commands.iter().enumerate() {
            self.results.push(BenchMark::new(number, cmd, self.opitons));
        }

        todo!()
    }
}
