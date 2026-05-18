use image_lib::{DynamicImage, GenericImageView as _};

use crate::types::{Color, Image};

#[derive(Clone, Default, Debug)]
pub struct SimpleImage {
    pub pixels: Vec<Color>,
    pub scansize: usize,
}

impl SimpleImage {
    pub fn new(pixels: Vec<Color>, scansize: usize) -> Self {
        SimpleImage { pixels, scansize }
    }
}

impl Image for SimpleImage {
    fn width(&self) -> u32 {
        self.scansize as u32
    }

    fn height(&self) -> u32 {
        (self.pixels.len() / self.scansize) as u32
    }

    fn sample(&self, x: u32, y: u32) -> Color {
        let idx = y * self.scansize as u32 + x;
        self.pixels[idx as usize]
    }

    fn into_buffer(self) -> Vec<u8> {
        let mut buffer = vec![];
        self.pixels.iter().for_each(|p| {
            buffer.push(p.r);
            buffer.push(p.g);
            buffer.push(p.b);
        });
        buffer
    }
}

impl From<DynamicImage> for SimpleImage {
    fn from(dynamic_image: DynamicImage) -> Self {
        let pixels = dynamic_image
            .pixels()
            .map(|p| Color::new(p.2.0[0], p.2.0[1], p.2.0[2]))
            .collect();
        let scansize = dynamic_image.width() as usize;
        SimpleImage { pixels, scansize }
    }
}
