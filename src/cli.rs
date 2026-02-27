//! Command-line interface
use anyhow::Result;
use clap::{Arg, ArgMatches, Command};

use helium::run_helium;

pub const RUN: &str = "run";

pub mod setup {
    use super::*;

    pub fn run() -> Command {
        Command::new(RUN).about("").args([Arg::new("verbose")
            .help("Enable printing logs to stdout")
            .long("verbose")
            .short('v')
            .action(clap::ArgAction::SetTrue)])
    }
}

pub mod run {
    use super::*;

    pub fn run(matches: &ArgMatches) -> Result<()> {
        if matches.get_flag("verbose") {
            tracing_subscriber::fmt::init();
        }

        run_helium()
    }
}
