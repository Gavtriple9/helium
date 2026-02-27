//! Command-line interface
use anyhow::Result;
use clap::{Arg, ArgMatches, Command};

use helium_core::App;
use winit::event_loop::EventLoop;

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

        let event_loop = EventLoop::new().unwrap();
        let mut app = App::new();
        event_loop.run_app(&mut app).unwrap();

        Ok(())
    }
}
