use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use crate::{
    algorithm::kmeans::{Centroid, kmeans},
    downsample::{Downsampler, SamplePattern},
    error::MigraineError,
    scale::{otsu, scale_guesser::ScaleGuesser as _},
    types::{Color, ColorFrequency, Image, Palette, SimpleImage},
};

pub struct MigraineResult {
    pub image: SimpleImage,
    pub palette: Palette,
    pub time_spent: Duration,
}

pub fn restore(
    image: impl Image,
    scale: Option<f64>,
    width: Option<u32>,
    height: Option<u32>,
    reduce_palette: bool,
    colors: Option<u32>,
) -> Result<MigraineResult, MigraineError> {
    let image_width = image.width() as f64;
    let image_height = image.height() as f64;

    let dimensions: (f64, f64, f64) = match (scale, width, height) {
        (None, None, None) => {
            let scale_guesser = otsu::OtsuGuesser::new();
            let scales = scale_guesser.guess(&image);
            let s = scales[0];
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

    let scale = dimensions.0;
    let target_width = dimensions.1;
    let target_height = dimensions.2;

    println!("Using scale {scale:?}");
    println!("Target width {target_width:?}");
    println!("Target height {target_height:?}");

    let downsampler = Downsampler::default();
    let sample_pattern = SamplePattern::combine(&SamplePattern::grid(), &SamplePattern::center());

    let start = Instant::now();

    let downsampled: SimpleImage = downsampler.downsample(
        &image,
        target_width.round() as u32,
        target_height.round() as u32,
        sample_pattern,
    );

    let (final_image, palette): (SimpleImage, Palette) = if reduce_palette {
        let palette = reduce(
            &downsampled.pixels,
            colors.expect("Guessing number of colors is not yet implemented"),
        );
        let reduced: Vec<Color> = downsampled
            .pixels
            .into_iter()
            .map(|p| palette.closest_to(&p))
            .collect();

        (SimpleImage::new(reduced, downsampled.scansize), palette)
    } else {
        let colors = downsampled.pixels.clone();
        (downsampled, Palette::new(colors))
    };

    let total = start.elapsed();

    Ok(MigraineResult {
        palette: palette,
        image: final_image,
        time_spent: total,
    })
}

pub fn reduce(colors: &[Color], palette_size: u32) -> Palette {
    let mut occurences: Vec<ColorFrequency> = colors
        .iter()
        .fold(HashMap::<Color, u32>::new(), |mut acc, c| {
            *acc.entry(*c).or_insert(0) += 1;
            acc
        })
        .iter()
        .map(ColorFrequency::from)
        .collect::<Vec<_>>();

    occurences.sort_by(|a, b| b.count.cmp(&a.count));

    let palette: Vec<Color> = kmeans(palette_size as usize, &occurences)
        .iter()
        .map(|c| ColorFrequency::centroid(c).color)
        .collect();

    Palette::new(palette)
}
