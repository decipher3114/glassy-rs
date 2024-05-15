use crate::utils::effect::EffectStrength;
use clap::{Parser, ValueHint};
use env_logger::Builder;
use log::{Level, LevelFilter};
use std::io::Write;
/// A simple CLI tool to apply glass-like overlay effect to images
#[derive(Debug, Parser)]
#[command(version, long_about = None)]
pub struct CliArgs {
    /// Path to image file
    #[arg(
        value_name = "PATH",
        value_hint = ValueHint::FilePath
    )]
    pub path: String,

    /// Strength of the glass effect
    #[arg(
        short,
        long,
        value_enum,
        default_value_t = EffectStrength::Medium
    )]
    pub effect_strength: EffectStrength,

    /// Apply effect without grain
    #[arg(long = "no-grain")]
    pub no_grain: bool,

    /// Specify output file path
    #[arg(short, long, value_name = "PATH", value_hint = ValueHint::FilePath)]
    pub output: Option<String>,

    /// Explain what is being done
    #[arg(short, long)]
    pub verbose: bool,
}

impl CliArgs {
    pub fn init_logger(&self) {
        let level = if self.verbose {
            LevelFilter::Error
        } else {
            LevelFilter::Info
        };

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
}
