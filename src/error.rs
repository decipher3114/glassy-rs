use image::ImageError as ImgError;
use std::io::Error as IoError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("No such file or directory")]
    FileNotFound(#[from] IoError),
    #[error("Invalid image file")]
    DecodeError(#[from] ImgError),
}
