use crate::utils::{blur::BlurOpts, noise::NoiseOpts};
use clap::ValueEnum;
use image::DynamicImage;
use std::{
    cmp::min,
    fmt::{Display, Formatter, Result},
};

#[derive(ValueEnum, Debug, Copy, Clone)]
pub enum EffectStrength {
    Low,
    Medium,
    High,
    Ultra,
}

impl EffectStrength {
    pub fn value(&self, img: &DynamicImage) -> (BlurOpts, NoiseOpts) {
        let min_dim = min(img.height(), img.width());
        match self {
            EffectStrength::Low => (
                BlurOpts {
                    sigma: min_dim as f32 * 0.02,
                },
                NoiseOpts {
                    mean: 0.00f64,
                    std_dev: min_dim as f64 * 0.005,
                    seed: (min_dim as f64 * 0.01) as u64,
                },
            ),
            EffectStrength::Medium => (
                BlurOpts {
                    sigma: min_dim as f32 * 0.04,
                },
                NoiseOpts {
                    mean: 0.00f64,
                    std_dev: min_dim as f64 * 0.0075,
                    seed: (min_dim as f64 * 0.01) as u64,
                },
            ),
            EffectStrength::High => (
                BlurOpts {
                    sigma: min_dim as f32 * 0.06,
                },
                NoiseOpts {
                    mean: 0.00f64,
                    std_dev: min_dim as f64 * 0.0100,
                    seed: (min_dim as f64 * 0.01) as u64,
                },
            ),
            EffectStrength::Ultra => (
                BlurOpts {
                    sigma: min_dim as f32 * 0.08,
                },
                NoiseOpts {
                    mean: 0.00f64,
                    std_dev: min_dim as f64 * 0.0125,
                    seed: (min_dim as f64 * 0.01) as u64,
                },
            ),
        }
    }
}

impl Display for EffectStrength {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            EffectStrength::Low => write!(f, "low"),
            EffectStrength::Medium => write!(f, "medium"),
            EffectStrength::High => write!(f, "high"),
            EffectStrength::Ultra => write!(f, "ultra"),
        }
    }
}
