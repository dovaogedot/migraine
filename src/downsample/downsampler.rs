use crate::{
    downsample::SamplePattern,
    types::{Color, Image, SimpleImage},
};

pub struct Downsampler {}
impl Downsampler {
    pub fn downsample(
        &self,
        img: &impl Image,
        target_width: u32,
        target_height: u32,
        sample_pattern: SamplePattern,
    ) -> SimpleImage {
        let pixel_w = img.width() as f64 / target_width as f64;
        let pixel_h = img.height() as f64 / target_height as f64;

        let mut pixels: Vec<Color> = vec![];

        for y in 0..target_height {
            for x in 0..target_width {
                let samples: Vec<Color> = sample_pattern
                    .positions
                    .iter()
                    .map(|(dx, dy)| {
                        let sx = x as f64 * pixel_w + pixel_w * 0.5 * (dx + 1.0);
                        let sy = y as f64 * pixel_h + pixel_h * 0.5 * (dy + 1.0);
                        img.sample(sx.round() as u32, sy.round() as u32)
                    })
                    .collect();

                let mut sum_r = 0u32;
                let mut sum_g = 0u32;
                let mut sum_b = 0u32;

                samples.iter().for_each(|c| {
                    sum_r += c.r as u32;
                    sum_g += c.g as u32;
                    sum_b += c.b as u32;
                });

                let total = sample_pattern.positions.len() as u32;

                let avg_r = (sum_r / total) as u8;
                let avg_g = (sum_g / total) as u8;
                let avg_b = (sum_b / total) as u8;

                let color = Color::new(avg_r, avg_g, avg_b);
                pixels.push(color)
            }
        }

        SimpleImage::new(pixels, target_width as usize)
    }
}

impl Default for Downsampler {
    fn default() -> Self {
        Self {}
    }
}
