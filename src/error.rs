use image::ImageError as ImgError;
use std::{io::Error as IoError, result::Result as StdResult};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("No such file or directory")]
    FileNotFound(#[from] IoError),
    #[error("Unable to decode image file")]
    DecodeError(#[from] ImgError),
}

pub type Result<T> = StdResult<T, Error>;