use std::path::Path;

use image::{DynamicImage, RgbImage};

pub struct IO;
impl IO {
    pub fn open_image(path: &Path) -> std::io::Result<DynamicImage> {
        image::ImageReader::open(path)?
            .with_guessed_format()?
            .decode()
            .map_err(|e| std::io::Error::other(format!("Could not decode image at path '{:?}'. {}", path, e)))
    }

    pub fn save_image(image: &RgbImage, path: &Path) -> std::io::Result<()> {
        image
            .save_with_format(path, image::ImageFormat::Bmp)
            .map_err(std::io::Error::other)
    }
}
