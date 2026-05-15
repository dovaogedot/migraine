use std::io::{Error, ErrorKind};

use clap::Parser;
use image_lib::{ImageReader, RgbImage};

use crate::{
    downsample::{Downsampler, SamplePattern},
    image::Image,
    scale::{guesser::ScaleGuesser, otsu},
};

mod color;
mod downsample;
mod image;
mod scale;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the upscaled version of the pixel art
    path: String,

    /// How many pixels in source image correspond to one pixel in original pixel art, can be fractional
    #[arg(short, long)]
    scale: Option<f64>,

    /// Original width of the pixel art
    #[arg(short, long)]
    width: Option<i16>,

    /// Original height of the pixel art
    #[arg(long)]
    height: Option<i16>,

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
    let sample_pattern = SamplePattern::grid();

    let downsampled = downsampler.downsample(img, target_width, target_height, sample_pattern);

    let new_path = format!("{}_downsampled.bmp", path);
    let buffer = downsampled.to_buffer();

    let rgb_image = RgbImage::from_fn(target_width, target_height, |x, y| {
        let color = downsampled.sample(x, y);
        image_lib::Rgb([color.r, color.g, color.b])
    });
    //     .ok_or(Error::new(ErrorKind::Other, "Container is not big enough"))?;
    rgb_image
        .save_with_format(new_path, image_lib::ImageFormat::Bmp)
        .expect("Could not save image");

    Ok(())
}
