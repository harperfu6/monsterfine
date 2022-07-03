use std::ffi::OsString;

use clap::{crate_version, AppSettings, ArgMatches, Command};

pub fn get_cli_arguments<I, T>(args: I) -> ArgMatches
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let command = build_command();
    command.get_matches_from(args)
}

fn build_command() -> Command<'static> {
    Command::new("monsterfine")
        .version(crate_version!())
        .setting(AppSettings::DeriveDisplayOrder)
        .next_line_help(true)
        .hide_possible_values(true)
        .max_term_width(90)
        .about("A SMALL command-line benchmarking tool.")
}
