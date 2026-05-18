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
                let mut sum_r = 0f64;
                let mut sum_g = 0f64;
                let mut sum_b = 0f64;

                let total_weight: f64 = sample_pattern.points.iter().map(|p| p.weight).sum();

                for point in &sample_pattern.points {
                    let sx = x as f64 * pixel_w + pixel_w * 0.5 * (point.dx + 1.0);
                    let sy = y as f64 * pixel_h + pixel_h * 0.5 * (point.dy + 1.0);
                    let color = img.sample(sx.round() as u32, sy.round() as u32);

                    sum_r += color.r * point.weight;
                    sum_g += color.g * point.weight;
                    sum_b += color.b * point.weight;
                }

                let color = Color::new(
                    sum_r / total_weight,
                    sum_g / total_weight,
                    sum_b / total_weight,
                );
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
