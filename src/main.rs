use clap::Parser;
use image_lib::RgbImage;

use crate::{
    color::Color,
    downsample::{Downsampler, SamplePattern},
    image::Image,
    palette::reduce_palette,
    scale::{guesser::ScaleGuesser, otsu},
};

mod color;
mod downsample;
mod image;
mod kmean;
mod palette;
mod scale;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the upscaled version of the pixel art
    path: String,

    #[arg(short, long)]
    colors: u32,

    /// How many pixels in source image correspond to one pixel in original pixel art, can be fractional
    #[arg(short, long)]
    scale: Option<f64>,

    /// Original width of the pixel art
    #[arg(short, long)]
    width: Option<u16>,

    /// Original height of the pixel art
    #[arg(long)]
    height: Option<u16>,

    /// Explains what's happenning
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let path = args.path;

    let img = &image_lib::ImageReader::open(&path)?
        .with_guessed_format()?
        .decode()
        .unwrap();

    let scale = args
        .scale
        .or_else(|| {
            let scale_guesser = otsu::OtsuGuesser {};
            let scales = scale_guesser.guess(img);
            scales.first().copied()
        })
        .expect("Failed to guess scale");

    println!("Using scale {scale:?}");

    let target_width = (img.width() as f64 / scale) as u32;
    let target_height = (img.height() as f64 / scale) as u32;

    let downsampler = Downsampler {};
    let sample_pattern = SamplePattern::combine(&SamplePattern::grid(), &SamplePattern::center());

    let downsampled = downsampler.downsample(img, target_width, target_height, sample_pattern);
    let palette = reduce_palette(&downsampled.pixels, args.colors);

    let new_path = format!("{}_downsampled.bmp", path);

    let rgb_image = RgbImage::from_fn(target_width, target_height, |x, y| {
        let color = downsampled.pixels[(y * target_width + x) as usize];
        let closest = palette
            .iter()
            .min_by(|c1, c2| c1.distance(&color).total_cmp(&c2.distance(&color)))
            .unwrap_or(&palette[0]);
        image_lib::Rgb([closest.r, closest.g, closest.b])
    });

    rgb_image
        .save_with_format(new_path, image_lib::ImageFormat::Bmp)
        .expect("Could not save image");

    Ok(())
}
