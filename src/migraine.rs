use std::{
    ops::{Div, Sub},
    time::{Duration, Instant},
};

use crate::{
    algorithm::{kmeans::Centroid, kmeans_pp::kmeans_pp},
    downsample::{Downsampler, SamplePattern},
    error::MigraineError,
    types::{Color, Image, Palette, SimpleImage},
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

    let downsampled: SimpleImage =
        downsampler.downsample(&image, target_width, target_height, sample_pattern);

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
    let palette: Vec<Color> = kmeans_pp(palette_size as usize, colors)
        .iter()
        .map(|c| Color::centroid(c))
        .collect();

    Palette::new(palette)
}

/// Guesses pixel size by shifting image horizontally and finding phases with smallest differences.
///
/// Turns out it's a variation of AMDF (Average Magnitude Difference Function) + Autocorrelation (YIN Algorithm)
///
/// TODO: cleanup space and time complexity
pub fn guess_pixel_size(img: &impl Image) -> f64 {
    let img_width = img.width();
    let img_height = img.height();
    let rows_to_sample = img.height().min(256);
    let dy = (img.height() / rows_to_sample) as usize;

    let rows: Vec<Vec<Color>> = (0..img_height)
        .step_by(dy)
        .map(|y| {
            (0..img_width)
                .map(move |x| img.sample(x, y.clone()))
                .collect()
        })
        .collect();

    // map pixel in each row to its color distance with the one next to it
    let distances: Vec<Vec<f64>> = rows
        .iter()
        .map(|row| {
            let mut r: Vec<f64> = row.windows(2).map(|w| w[0].distance(&w[1])).collect();
            r.push(row[row.len() - 1].distance(&row[0]));
            r
        })
        .collect();

    // average distance to next pixel color in each column
    let avg_distances: Vec<f64> = (0..distances[0].len())
        .map(|i| {
            distances
                .iter()
                .map(|row| row[i])
                .sum::<f64>()
                .div(distances.len() as f64)
        })
        .collect();

    let mut phase_min_diff = f64::MAX;
    let mut phase_max_diff = f64::MIN;

    // for each column calculate difference between original graph and the one shifted by column index
    let phase_differences: Vec<f64> = (0..avg_distances.len() / 2)
        .map(|phase| {
            let difference = (0..avg_distances.len())
                .map(|current| {
                    avg_distances[current]
                        .sub(avg_distances[(current + phase) % avg_distances.len()])
                        .powi(2)
                })
                .sum::<f64>()
                .div(avg_distances.len() as f64);
            phase_max_diff = phase_max_diff.max(difference);
            phase_min_diff = phase_min_diff.min(difference);
            difference
        })
        .collect();

    // skip fluctuations below this threshold
    let tolerance = (phase_max_diff - phase_min_diff) * 0.001;
    println!("min: {}, max: {}", phase_min_diff, phase_max_diff);

    // find spikes where difference is the lowest
    let local_minimums: Vec<usize> = phase_differences
        .windows(3)
        .enumerate()
        .filter_map(|(i, w)| {
            let left = w[0];
            let mid = w[1];
            let right = w[2];

            if left.sub(mid).gt(&tolerance) && right.sub(mid).gt(&tolerance) {
                Some(i + 1)
            } else {
                None
            }
        })
        .collect();

    // sum up distances between spikes
    let pixel_size_sum = local_minimums
        .windows(2)
        .map(|w| w[1] - w[0])
        .sum::<usize>() as f64;

    // get average distance
    let pixel_size = pixel_size_sum.div((local_minimums.len() - 1) as f64);

    pixel_size
}
