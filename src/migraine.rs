use std::{
    ops::{Div, Mul},
    time::{Duration, Instant},
};

use image::{Rgb, Rgb32FImage, RgbImage};

use crate::{
    algorithm::{
        autocorrelation::{Period, YIN},
        kmeans::Distance,
    },
    color::palette::Palette,
    downsample::{Downsampler, SamplePattern},
    error::MigraineError,
    visualizer::send_to_visualizer,
};

pub struct MigraineResult {
    pub image: RgbImage,
    pub palette: Palette,
    pub time_spent: Duration,
}

pub fn restore(
    image: &Rgb32FImage,
    scale: Option<f64>,
    width: Option<u32>,
    height: Option<u32>,
    colors: Option<u32>,
    max_colors: Option<u32>,
) -> Result<MigraineResult, MigraineError> {
    let image_width = image.width() as f64;
    let image_height = image.height() as f64;

    let (scale, width, height): (f64, f64, f64) = match (scale, width, height) {
        (None, None, None) => {
            let s = guess_pixel_size(&image);
            let w = image_width / s;
            let h = image_height / s;
            (s, w, h)
        }
        (None, None, Some(h)) => {
            let h = h as f64;
            let s = image_height / h;
            let w = image_width / s;
            (s, w, h)
        }
        (None, Some(w), None) => {
            let w = w as f64;
            let s = image_width / w;
            let h = image_height / s;
            (s, w, h)
        }
        (None, Some(w), Some(h)) => {
            let w = w as f64;
            let h = h as f64;
            let s = (image_width / w + image_height / h) / 2.0;
            (s, w, h)
        }
        (Some(s), None, None) => {
            let w = image_width / s;
            let h = image_height / s;
            (s, w, h)
        }
        (_, _, _) => return Err(MigraineError::SuppliedBothDimensionsAndScale),
    };

    let target_width = width.round() as u32;
    let target_height = height.round() as u32;

    println!("Using scale {scale:?}");
    println!("Target width {target_width:?}");
    println!("Target height {target_height:?}");

    let downsampler = Downsampler::default();
    let sample_pattern = SamplePattern::default();

    let start = Instant::now();

    let downsampled = downsampler.downsample(&image, target_width, target_height, sample_pattern);

    let palette = Palette::new(
        downsampled
            .pixels()
            .map(|p| Rgb::<f64>::from([p.0[0] as f64, p.0[1] as f64, p.0[2] as f64]))
            .collect(),
    );

    let data: Vec<[f32; 3]> = downsampled.pixels().map(|p| p.0).collect();
    send_to_visualizer(&data);

    let reduced_palette = match colors {
        None => palette.reduced_auto(max_colors),
        Some(n) => palette.reduced(n),
    };

    let posterized_pixels: Vec<u8> = palette
        .colors()
        .iter()
        .flat_map(|p| {
            let color = reduced_palette.closest_to(&p);
            [
                color.0[0].mul(255.0).round() as u8,
                color.0[1].mul(255.0).round() as u8,
                color.0[2].mul(255.0).round() as u8,
            ]
        })
        .collect();

    let restored = RgbImage::from_raw(downsampled.width(), downsampled.height(), posterized_pixels).unwrap();

    let total = start.elapsed();

    Ok(MigraineResult {
        palette: reduced_palette,
        image: restored,
        time_spent: total,
    })
}

/// Guesses pixel size by transforming the image into a function of differences
/// between two adjacent pixels and then applying YIN autocorrelation function
/// to find its perid.
pub fn guess_pixel_size(image: &Rgb32FImage) -> f64 {
    let img_width = image.width();
    let img_height = image.height();
    let rows_to_sample = img_height.min(256);
    let dy = (img_height / rows_to_sample) as usize;

    // Map pixel in each row to its color distance with the one next to it
    let distances: Vec<f64> = (0..img_width)
        .map(|x| {
            (0..img_height)
                .step_by(dy)
                .map(|y| {
                    let left = image.get_pixel(x, y);
                    let right = if x + 1 == img_width {
                        image.get_pixel(0, y)
                    } else {
                        image.get_pixel(x + 1, y)
                    };

                    left.distance(right)
                })
                .sum::<f64>()
                .div(rows_to_sample as f64)
        })
        .collect();

    YIN::period(&distances)
}
