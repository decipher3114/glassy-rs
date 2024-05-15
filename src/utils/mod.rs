use crate::error::Result;
use self::{blur::add_blur, noise::add_noise};
use image::io::Reader;
use log::info;
use std::{fs::DirBuilder, path::Path};

mod blur;
pub mod cli;
mod effect;
mod noise;

pub fn proc_image(cli_args: cli::CliArgs) -> Result<()> {
    let path = Path::new(&cli_args.path);
    info!("Reading File: \"{}\"", path.display());

    let img = Reader::open(path)?.decode()?;
    let (blur_opts, noise_opts) = cli_args.effect_strength.value(&img);

    info!("Applying Blur...");

    let img = add_blur(img, blur_opts);
    let img = if cli_args.no_grain {
        img
    } else {
        info!("Applying Noise...");
        add_noise(img, noise_opts)
    };

    let output_path = if let Some(output) = cli_args.output {
        output
    } else {
        format!(
            "{}_{}.{}",
            path.file_stem().unwrap().to_str().unwrap_or("Image"),
            cli_args.effect_strength,
            path.extension().unwrap().to_str().unwrap_or("png")
        )
    };

    // Create the output directory if it doesn't exist
    let output = Path::new(output_path.as_str());
    if !output.exists() {
        DirBuilder::new()
            .recursive(true)
            .create(output.parent().unwrap())?;
    }

    info!("Saving Image: {output_path}");

    img.save(output_path)?;

    info!("Operation Completed !!");

    Ok(())
}
