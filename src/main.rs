use crate::utils::cli::CliArgs;
use clap::Parser;
use log::error;
use std::process::exit;

mod error;
mod utils;

fn main() {
    let cli_args: CliArgs = CliArgs::parse();

    cli_args.init_logger();

    if let Err(e) = cli_args.proc_image() {
        error!("{e}");
        exit(1);
    };
}
