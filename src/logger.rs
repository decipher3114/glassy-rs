use crate::utils::cli::CliArgs;
use env_logger::Builder;
use log::{Level, LevelFilter};
use std::io::Write;

pub fn init_logger(cli_args: &CliArgs) {
    let mut level = log::LevelFilter::Error;

    if cli_args.verbose {
        level = LevelFilter::Info;
    }

    Builder::new()
        .format(|buf, record| {
            let info_style = buf.default_level_style(Level::Info);
            let error_style = buf.default_level_style(Level::Error);

            match record.level() {
                Level::Error => writeln!(
                    buf,
                    "{error_style}{}{error_style:#}: {}",
                    record.level(),
                    record.args()
                ),
                Level::Info => writeln!(
                    buf,
                    "{info_style}{}{info_style:#}: {}",
                    record.level(),
                    record.args()
                ),
                _ => writeln!(buf, "{}: {}", record.level(), record.args()),
            }
        })
        .filter(None, level)
        .init();
}
