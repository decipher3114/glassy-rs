use crate::utils::cli::CliArgs;
use std::io::Write;

pub fn init_logger(cli_args: &CliArgs) {
    let mut level = log::LevelFilter::Error;

    if cli_args.verbose {
        level = log::LevelFilter::Info;
    }

    env_logger::Builder::new()
    .format(|buf, record| {
        let info_style = buf.default_level_style(log::Level::Info);
        let error_style = buf.default_level_style(log::Level::Error);

        match record.level() {
            log::Level::Error => writeln!(
                buf,
                "{error_style}{}{error_style:#}: {}",
                record.level(),
                record.args()
            ),
            log::Level::Info => writeln!(
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