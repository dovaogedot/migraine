use image_lib::{DynamicImage, GenericImageView, Pixel};

use crate::types::Color;

/// Provides essential methods used to process an image.
pub trait Image {
    /// Returns width of the image.
    fn width(&self) -> u32;

    /// Returns height of the image.
    fn height(&self) -> u32;

    /// Returns pixel color value at position (x, y).
    fn sample(&self, x: u32, y: u32) -> Color;

    fn into_buffer(self) -> Vec<u8>;
}

impl Image for DynamicImage {
    fn width(&self) -> u32 {
        self.width()
    }

    fn height(&self) -> u32 {
        self.height()
    }

    fn sample(&self, x: u32, y: u32) -> Color {
        let p = self.get_pixel(x, y).to_rgb();
        Color {
            r: p[0],
            g: p[1],
            b: p[2],
        }
    }

    fn into_buffer(self) -> Vec<u8> {
        self.into_rgb8().into_raw()
    }
}
