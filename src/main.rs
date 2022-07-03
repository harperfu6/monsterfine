use clap::{arg, Command};
use cli::get_cli_arguments;
use std::env;

mod benchmark;
mod cli;
mod command;
mod options;
mod parameter;
mod util;

fn main() {
    let cli_arguments = get_cli_arguments(env::args_os());
}
