#[cfg(not(any(target_os = "android", target_os = "ios")))]
mod cli;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use anyhow::Result;
#[cfg(not(any(target_os = "android", target_os = "ios")))]
use clap::Command;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
const PROGRAM_NAME: &str = "helium";
#[cfg(not(any(target_os = "android", target_os = "ios")))]
const PROGRAM_VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
#[cfg(not(any(target_os = "android", target_os = "ios")))]
const PROGRAM_DESCRIPTION: &str = "A GPU-accelerated math object viewer built in Rust";

#[cfg(not(any(target_os = "android", target_os = "ios")))]
fn main() -> Result<()> {
    let command = Command::new(PROGRAM_NAME)
        .version(PROGRAM_VERSION.unwrap_or("unknown"))
        .about(PROGRAM_DESCRIPTION)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommands([cli::setup::run()]);

    let matches = command.get_matches();
    match matches.subcommand() {
        Some((cli::RUN, matches)) => cli::run::run(matches)?,
        _ => unreachable!(),
    }

    Ok(())
}

#[cfg(any(target_os = "android", target_os = "ios"))]
fn main() {}
