use std::{
    ops::Div,
    time::{Duration, Instant},
};

use image_lib::{Rgb, Rgb32FImage};

use crate::{
    algorithm::{
        autocorrelation::{Period, YIN},
        kmeans::Distance,
    },
    downsample::{Downsampler, SamplePattern},
    error::MigraineError,
    types::palette::Palette,
};

pub struct MigraineResult {
    pub image: Rgb32FImage,
    pub palette: Palette,
    pub time_spent: Duration,
}

pub fn restore(
    image: &Rgb32FImage,
    scale: Option<f64>,
    width: Option<u32>,
    height: Option<u32>,
    reduce_palette: bool,
    colors: Option<u32>,
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

    let (downsampled, palette) = if reduce_palette {
        let reduced_palette = palette.reduced(colors.expect("Guessing number of colors is not yet implemented"));

        let posterized_pixels: Vec<f32> = palette
            .colors
            .iter()
            .flat_map(|p| {
                let color = reduced_palette.closest_to(&p);
                [color.0[0] as f32, color.0[1] as f32, color.0[2] as f32]
            })
            .collect();
        (
            Rgb32FImage::from_raw(downsampled.width(), downsampled.height(), posterized_pixels).unwrap(),
            reduced_palette,
        )
    } else {
        (downsampled, palette)
    };

    let total = start.elapsed();

    Ok(MigraineResult {
        palette: palette,
        image: downsampled,
        time_spent: total,
    })
}

/// Guesses pixel size by treating image as a function of difference between two
/// adjacent pixels and then applying YIN autocorrelation function to find its
/// perid.
pub fn guess_pixel_size(image: &Rgb32FImage) -> f64 {
    let img_width = image.width();
    let img_height = image.height();
    let rows_to_sample = img_height.min(256);
    let dy = (img_height / rows_to_sample) as usize;

    // map pixel in each row to its color distance with the one next to it
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

                    Distance::distance(left, right)
                })
                .sum::<f64>()
                .div(rows_to_sample as f64)
        })
        .collect();

    YIN::period(&distances)
}
