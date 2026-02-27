mod cli;

use anyhow::Result;
use clap::Command;

const HELIUM_VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> Result<()> {
    let command = Command::new("helium")
        .version(HELIUM_VERSION.unwrap_or("unknown"))
        .about("A GPU-accelerated math object viewer built in Rust")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommands([cli::setup::run()]);

    let matches = command.get_matches();
    match matches.subcommand() {
        Some((cli::RUN, matches)) => cli::run::run(matches).await?,
        _ => unreachable!(),
    }

    Ok(())
}
