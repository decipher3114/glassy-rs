use crate::utils::effect::EffectStrength;
use clap::Parser;

/// A simple CLI tool to apply glass-like overlay effect to images
#[derive(Debug, Parser)]
#[command(version, long_about = None)]
pub struct CliArgs {
    /// Path to image file
    #[arg(
        value_name = "PATH",
        value_hint = clap::ValueHint::FilePath
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
    #[arg(
        long="no-grain"
    )]
    pub no_grain: bool,

    /// Explain what is being done
    #[arg(short, long)]
    pub verbose: bool,
}