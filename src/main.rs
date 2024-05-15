use crate::utils::{cli::CliArgs, proc_image};
use clap::Parser;
use log::error;
use std::process::exit;

mod error;
mod utils;

fn main() {
    let cli_args: CliArgs = CliArgs::parse();

    cli_args.init_logger();

    if let Err(e) = proc_image(cli_args) {
        error!("{e}");
        exit(1);
    };
}
