use image::{DynamicImage, RgbImage};
use libblur::{fast_gaussian, FastBlurChannels};

pub struct BlurOpts {
    pub sigma: f32,
}

pub fn add_blur(img: DynamicImage, blur_opts: BlurOpts) -> DynamicImage {
    let height: u32 = img.height();

    let width: u32 = img.width();

    let mut raw_img: Vec<u8> = DynamicImage::to_rgb8(&img).into_raw();

    fast_gaussian(
        &mut raw_img,
        width * 3,
        width,
        height,
        blur_opts.sigma as u32,
        FastBlurChannels::Channels3,
    );

    let final_img = RgbImage::from_raw(width, height, raw_img).unwrap();

    DynamicImage::from(final_img)
}
