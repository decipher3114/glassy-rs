use crate::{
    logger::init_logger,
    utils::{cli::CliArgs, proc_image},
};
use clap::Parser;
use log::error;
use std::process::exit;

mod error;
mod logger;
mod utils;

fn main() {
    let cli_args: CliArgs = CliArgs::parse();

    init_logger(&cli_args);

    if let Err(e) = proc_image(cli_args) {
        error!("{e}");
        exit(1);
    };
}
