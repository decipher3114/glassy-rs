use clap::Parser;

use crate::utils::{cli::CliArgs, proc_image};
use std::process;

mod utils;
mod error;
mod logger;

fn main() {
    let cli_args: CliArgs = CliArgs::parse();

    logger::init_logger(&cli_args);

    if let Err(e) = proc_image(cli_args) {
        log::error!("{e}");
        process::exit(1);
    };
}
