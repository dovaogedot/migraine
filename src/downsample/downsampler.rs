use std::marker::PhantomData;

use image::Rgb32FImage;

use crate::downsample::SamplePattern;

pub struct Downsampler {
    _private: PhantomData<bool>,
}

impl Downsampler {
    pub fn new() -> Self {
        Downsampler { _private: PhantomData }
    }

    pub fn downsample(
        &self,
        image: &Rgb32FImage,
        target_width: u32,
        target_height: u32,
        sample_pattern: SamplePattern,
    ) -> Rgb32FImage {
        let pixel_w = image.width() as f64 / target_width as f64;
        let pixel_h = image.height() as f64 / target_height as f64;

        Rgb32FImage::from_fn(target_width, target_height, |x, y| {
            let mut sum_r = 0f64;
            let mut sum_g = 0f64;
            let mut sum_b = 0f64;

            let total_weight: f64 = sample_pattern.points().iter().map(|p| p.weight).sum();

            for point in sample_pattern.points() {
                let sample_pos_x = x as f64 * pixel_w + pixel_w * 0.5 * (point.dx + 1.0);
                let sample_pos_y = y as f64 * pixel_h + pixel_h * 0.5 * (point.dy + 1.0);
                let sample_color = image.get_pixel(sample_pos_x.round() as u32, sample_pos_y.round() as u32);

                sum_r += sample_color.0[0] as f64 * point.weight;
                sum_g += sample_color.0[1] as f64 * point.weight;
                sum_b += sample_color.0[2] as f64 * point.weight;
            }

            let r = sum_r / total_weight;
            let g = sum_g / total_weight;
            let b = sum_b / total_weight;
            [r as f32, g as f32, b as f32].into()
        })
    }
}

impl Default for Downsampler {
    fn default() -> Self {
        Self::new()
    }
}
