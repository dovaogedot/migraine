use image_lib::{DynamicImage, GenericImageView, Pixel};

use crate::color::Color;

pub trait Image {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn sample(&self, x: u32, y: u32) -> Color;
    fn to_buffer(&self) -> Vec<u8>;
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

    fn to_buffer(&self) -> Vec<u8> {
        todo!("Image::buffer not implemented for DynamicImage")
    }
}

pub struct SimpleImage {
    pixels: Vec<Color>,
    scansize: usize,
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
    
    fn to_buffer(&self) -> Vec<u8> {
        self.pixels.iter().flat_map(|c| [c.r, c.g, c.b]).collect()
    }
}
