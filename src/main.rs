use std::path::Path;

use clap::Parser;
use image_lib::{DynamicImage, RgbImage};

use crate::{
    args::MigraineArgs,
    types::{Image, SimpleImage},
};

mod algorithm;
mod args;
mod downsample;
mod error;
mod migraine;
mod scale;
mod types;

fn main() -> std::io::Result<()> {
    let args = MigraineArgs::parse();

    let image = open_image(&args.path)?;

    let result = migraine::restore(image, args.scale, args.width, args.height, args.reduce_palette, args.colors)?;

    println!("Processing took {}ms", result.time_spent.as_millis());
    println!("Approximated palette:\n{}", result.palette);

    let new_path_str = format!("{}_downsampled.bmp", args.path.to_string_lossy());
    let new_path = Path::new(&new_path_str);
    save_image(result.image, &new_path)
}

fn open_image(path: &Path) -> std::io::Result<DynamicImage> {
    Ok(image_lib::ImageReader::open(path)?
        .with_guessed_format()?
        .decode()
        .unwrap())
}

fn save_image(image: SimpleImage, path: &Path) -> std::io::Result<()> {
    let result_width = image.width();
    let result_height = image.height();
    let buffer = image.into_buffer();

    let rgb_image = RgbImage::from_raw(result_width, result_height, buffer)
        .expect("Buffer length does not agree with provided dimensions");

    rgb_image
        .save_with_format(path, image_lib::ImageFormat::Bmp)
        .map_err(std::io::Error::other)?;

    Ok(())
}
