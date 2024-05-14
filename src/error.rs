use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("No such file or directory")]
    FileNotFound(#[from] std::io::Error),
    #[error("Invalid image file")]
    DecodeError(#[from] image::ImageError),
}

pub type Result<T> = std::result::Result<T, Error>;
