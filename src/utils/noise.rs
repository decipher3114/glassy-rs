use image::DynamicImage;
use imageproc::noise::gaussian_noise;

pub struct NoiseOpts {
    pub mean: f64,
    pub std_dev: f64,
    pub seed: u64,
}

pub fn add_noise(img: DynamicImage, noise_opts: NoiseOpts) -> DynamicImage {
    DynamicImage::from(gaussian_noise(
        &DynamicImage::into_rgb8(img),
        noise_opts.mean,
        noise_opts.std_dev,
        noise_opts.seed,
    ))
}
