#[derive(Debug)]
pub enum MigraineError {
    SuppliedBothDimensionsAndScale,
    ImageError(image_lib::ImageError),
    IoError(std::io::Error),
}

impl From<image_lib::ImageError> for MigraineError {
    fn from(value: image_lib::ImageError) -> Self {
        MigraineError::ImageError(value)
    }
}

impl From<std::io::Error> for MigraineError {
    fn from(value: std::io::Error) -> Self {
        MigraineError::IoError(value)
    }
}

impl From<MigraineError> for std::io::Error {
    fn from(value: MigraineError) -> Self {
        match value {
            MigraineError::IoError(e) => e,
            MigraineError::ImageError(image_error) => std::io::Error::other(image_error),
            MigraineError::SuppliedBothDimensionsAndScale => std::io::Error::other(
                "You have to supply either pixel scale or original art dimensions",
            ),
        }
    }
}
