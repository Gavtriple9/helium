//! Command-line interface
use anyhow::Result;
use clap::{Arg, ArgMatches, Command};

use helium_core::App;
use winit::event_loop::{ControlFlow, EventLoop};

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

    pub async fn run(matches: &ArgMatches) -> Result<()> {
        if matches.get_flag("verbose") {
            tracing_subscriber::fmt::init();
        }

        let event_loop = EventLoop::with_user_event().build()?;
        // Since we don't need to do anything when idle, set the control flow to Wait
        event_loop.set_control_flow(ControlFlow::Wait);

        let mut app = App::new();
        event_loop.run_app(&mut app).unwrap();

        Ok(())
    }
}
