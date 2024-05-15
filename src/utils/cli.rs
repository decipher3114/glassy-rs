use crate::{
    error::Result,
    utils::{blur::add_blur, effect::EffectStrength, noise::add_noise},
};
use clap::{Parser, ValueHint};
use env_logger::Builder;
use image::io::Reader;
use log::{info, Level, LevelFilter};
use std::{fs::DirBuilder, io::Write, path::Path};

const DEFAULT_FALLBACK_OUTPUT_DIR: &str = "./";

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

    pub fn proc_image(self) -> Result<()> {
        let path = Path::new(&self.path);
        info!("Reading File: \"{}\"", path.display());

        let img = Reader::open(path)?.decode()?;
        let (blur_opts, noise_opts) = self.effect_strength.value(&img);

        info!("Applying Blur...");

        let img = add_blur(img, blur_opts);
        let img = if self.no_grain {
            img
        } else {
            info!("Applying Noise...");
            add_noise(img, noise_opts)
        };

        let output_path = if let Some(output) = self.output {
            // Create the output directory if it doesn't exist
            let output_path = Path::new(output.as_str());
            if !output_path.exists() {
                DirBuilder::new().recursive(true).create(
                    output_path
                        .parent()
                        .unwrap_or(Path::new(DEFAULT_FALLBACK_OUTPUT_DIR)),
                )?;
            }
            output
        } else {
            format!(
                "{}/{}_{}.{}",
                path.parent().unwrap().to_str().unwrap_or("."),
                path.file_stem().unwrap().to_str().unwrap_or("Image"),
                self.effect_strength,
                path.extension().unwrap().to_str().unwrap_or("png")
            )
        };

        info!("Saving Image: {output_path}");

        img.save(output_path)?;

        info!("Operation Completed !!");

        Ok(())
    }
}
