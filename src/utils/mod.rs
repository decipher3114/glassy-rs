use crate::error;
use image::io::Reader;
use std::path::Path;

pub mod cli;
mod blur;
mod noise;
mod effect;

pub fn proc_image(cli_args: cli::CliArgs) -> error::Result<()> {

    let path = Path::new(&cli_args.path);

    log::info!("Reading File: \"{}\"", path.display());

    let img = Reader::open(path)?.decode()?;

    let (blur_opts, noise_opts) = cli_args.effect_strength.value(&img);

    log::info!("Applying Blur...");

    let img = blur::add_blur(img, blur_opts)?;

    let img = if ! cli_args.no_grain {
        
        log::info!("Applying Noise...");

        noise::add_noise(img, noise_opts)?
    } else {
        img
    };

    let output_path = format!(
        "{}_{}.{}",
        path.file_stem().unwrap().to_str().unwrap_or("Image"),
        cli_args.effect_strength,
        path.extension().unwrap().to_str().unwrap_or("png")
    );

    log::info!("Saving Image: {output_path}");

    img.save(output_path)?;

    log::info!("Operation Completed !!");

    Ok(())
}
